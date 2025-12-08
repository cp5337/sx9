#!/usr/bin/env python3
"""
ABE Linear QA Integration Daemon
Connects QA results to Linear issues and PR automation
Integrates with existing CTAS Linear service and repo agents
"""

import asyncio
import json
import logging
import os
from datetime import datetime
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, asdict
import httpx
import structlog
from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.responses import JSONResponse

# Load environment from command center
from dotenv import load_dotenv
load_dotenv('/Users/cp5337/Developer/ctas7-command-center/.env')

logger = structlog.get_logger()

@dataclass
class LinearIssue:
    """Linear issue for QA findings"""
    title: str
    description: str
    priority: int  # 1-4 (1=urgent, 4=low)
    team_id: str
    project_id: Optional[str]
    labels: List[str]
    assignee_id: Optional[str]
    qa_source: str  # "lightning" or "expert"
    crate_name: str
    issue_type: str  # "bug", "improvement", "task"

@dataclass
class PRAutomation:
    """PR automation task for Claude meta-agents"""
    title: str
    description: str
    branch_name: str
    files_to_modify: List[str]
    suggested_changes: List[Dict]
    priority: str
    estimated_effort: str
    claude_agent_type: str  # "code_review", "documentation", "testing"

class LinearQADaemon:
    """
    Linear QA Integration Daemon
    Connects to existing CTAS Linear service and repo agents
    """

    def __init__(self):
        self.app = FastAPI(
            title="ABE Linear QA Integration",
            description="Linear issue creation from QA results",
            version="1.0.0"
        )
        self.setup_routes()

        # Get config from environment
        self.linear_api_key = os.getenv("LINEAR_API_KEY")
        self.linear_team_id = os.getenv("LINEAR_TEAM_ID")
        self.github_token = os.getenv("GITHUB_TOKEN")
        self.port_manager_url = os.getenv("PORT_MANAGER_URL", "http://localhost:18103")
        self.lightning_qa_url = "http://localhost:18109"
        self.expert_qa_url = "http://localhost:18110"

        logger.info("Linear QA Daemon initialized",
                   has_linear_key=bool(self.linear_api_key),
                   team_id=self.linear_team_id)

    def setup_routes(self):
        """Setup FastAPI routes"""

        @self.app.get("/health")
        async def health_check():
            return JSONResponse({
                "status": "healthy",
                "service": "linear-qa-integration",
                "linear_connected": bool(self.linear_api_key),
                "team_id": self.linear_team_id,
                "timestamp": datetime.utcnow().isoformat()
            })

        @self.app.post("/process-qa-results/{crate_name}")
        async def process_qa_results(crate_name: str, background_tasks: BackgroundTasks):
            """Process QA results and create Linear issues + PR tasks"""
            try:
                background_tasks.add_task(self._process_crate_qa, crate_name)
                return JSONResponse({
                    "status": "processing",
                    "crate_name": crate_name,
                    "message": "QA results processing for Linear integration"
                })
            except Exception as e:
                logger.error("QA processing failed", error=str(e))
                raise HTTPException(status_code=500, detail=str(e))

        @self.app.post("/create-issue")
        async def create_linear_issue(issue_data: dict):
            """Create a Linear issue from QA findings"""
            try:
                linear_issue = LinearIssue(**issue_data)
                issue_id = await self._create_linear_issue(linear_issue)
                return JSONResponse({
                    "status": "created",
                    "issue_id": issue_id,
                    "linear_url": f"https://linear.app/issue/{issue_id}"
                })
            except Exception as e:
                logger.error("Issue creation failed", error=str(e))
                raise HTTPException(status_code=500, detail=str(e))

        @self.app.post("/create-pr-task")
        async def create_pr_automation_task(pr_data: dict):
            """Create PR automation task for Claude meta-agents"""
            try:
                pr_task = PRAutomation(**pr_data)
                task_id = await self._create_pr_task(pr_task)
                return JSONResponse({
                    "status": "created",
                    "task_id": task_id,
                    "claude_agent": pr_task.claude_agent_type
                })
            except Exception as e:
                logger.error("PR task creation failed", error=str(e))
                raise HTTPException(status_code=500, detail=str(e))

        @self.app.get("/linear/teams")
        async def get_linear_teams():
            """Get Linear teams for configuration"""
            try:
                teams = await self._get_linear_teams()
                return JSONResponse({"teams": teams})
            except Exception as e:
                raise HTTPException(status_code=500, detail=str(e))

        @self.app.post("/register")
        async def register_with_port_manager():
            """Register with the Port Manager"""
            try:
                async with httpx.AsyncClient() as client:
                    response = await client.post(
                        f"{self.port_manager_url}/register",
                        json={
                            "service_name": "linear-qa-integration",
                            "port": 18111,
                            "layer": "integration",
                            "capabilities": ["linear_issues", "pr_automation", "repo_agents"],
                            "health_endpoint": "/health"
                        }
                    )
                    if response.status_code == 200:
                        logger.info("Successfully registered with Port Manager")
                        return {"status": "registered"}
                    else:
                        logger.error("Failed to register with Port Manager", status_code=response.status_code)
                        return {"status": "registration_failed"}
            except Exception as e:
                logger.error("Registration error", error=str(e))
                return {"status": "error", "message": str(e)}

    async def _process_crate_qa(self, crate_name: str):
        """Process QA results for a crate and create Linear issues + PR tasks"""
        logger.info("Processing QA results", crate_name=crate_name)

        try:
            # Get Lightning QA results (Layer 1)
            lightning_results = await self._get_lightning_results(crate_name)

            # Get Expert QA results (Layer 2)
            expert_results = await self._get_expert_results(crate_name)

            # Create Linear issues from critical findings
            await self._create_issues_from_qa(crate_name, lightning_results, expert_results)

            # Create PR automation tasks for Claude meta-agents
            await self._create_pr_tasks_from_qa(crate_name, lightning_results, expert_results)

            logger.info("QA processing completed", crate_name=crate_name)

        except Exception as e:
            logger.error("QA processing failed", crate_name=crate_name, error=str(e))

    async def _get_lightning_results(self, crate_name: str) -> Optional[Dict]:
        """Get Lightning QA results from Layer 1"""
        try:
            async with httpx.AsyncClient() as client:
                response = await client.get(f"{self.lightning_qa_url}/results/{crate_name}")
                if response.status_code == 200:
                    return response.json()
        except Exception as e:
            logger.warning("Could not get Lightning QA results", error=str(e))
        return None

    async def _get_expert_results(self, crate_name: str) -> Optional[Dict]:
        """Get Expert QA results from Layer 2"""
        try:
            async with httpx.AsyncClient() as client:
                response = await client.get(f"{self.expert_qa_url}/results/expert/{crate_name}")
                if response.status_code == 200:
                    return response.json()
        except Exception as e:
            logger.warning("Could not get Expert QA results", error=str(e))
        return None

    async def _create_issues_from_qa(self, crate_name: str, lightning_results: Dict, expert_results: Dict):
        """Create Linear issues from QA findings"""
        issues_created = 0

        # Process Lightning QA critical issues
        if lightning_results and "critical_issues" in lightning_results:
            for issue in lightning_results["critical_issues"]:
                if issue.get("severity") in ["high", "critical"]:
                    linear_issue = LinearIssue(
                        title=f"[QA-Lightning] {issue['description']} in {crate_name}",
                        description=f"""
## Lightning QA Finding
**Crate**: {crate_name}
**Type**: {issue.get('type', 'unknown')}
**Severity**: {issue.get('severity', 'unknown')}

### Description
{issue['description']}

### Analysis Context
- Overall Grade: {lightning_results.get('overall_grade', 'N/A')}
- Security Score: {lightning_results.get('security_score', 'N/A')}/10
- Complexity Score: {lightning_results.get('complexity_score', 'N/A')}/10

### Source
Generated by ABE Lightning QA Engine (Layer 1)
                        """.strip(),
                        priority=1 if issue.get('severity') == 'critical' else 2,
                        team_id=self.linear_team_id,
                        project_id=None,
                        labels=["qa", "lightning", issue.get('type', 'unknown')],
                        assignee_id=None,
                        qa_source="lightning",
                        crate_name=crate_name,
                        issue_type="bug" if issue.get('type') == 'security' else "task"
                    )

                    await self._create_linear_issue(linear_issue)
                    issues_created += 1

        # Process Expert QA findings
        if expert_results and "code_review_findings" in expert_results:
            for finding in expert_results["code_review_findings"]:
                if finding.get("severity") in ["high", "critical"]:
                    linear_issue = LinearIssue(
                        title=f"[QA-Expert] {finding['description']} in {crate_name}",
                        description=f"""
## Expert QA Finding
**Crate**: {crate_name}
**File**: {finding.get('file', 'unknown')}
**Line**: {finding.get('line', 'N/A')}
**Category**: {finding.get('category', 'unknown')}
**Severity**: {finding.get('severity', 'unknown')}
**AI Model**: {finding.get('ai_model', 'unknown')}
**Confidence**: {finding.get('confidence', 0.0):.2f}

### Description
{finding['description']}

### Suggested Fix
{finding.get('suggestion', 'No suggestion provided')}

### Expert Assessment
- Platform Expertise Score: {expert_results.get('platform_expertise_score', 'N/A')}/10
- Tests Generated: {expert_results.get('automated_tests_generated', 0)}

### Source
Generated by ABE ZenCoder Expert QA (Layer 2)
                        """.strip(),
                        priority=1 if finding.get('severity') == 'critical' else 2,
                        team_id=self.linear_team_id,
                        project_id=None,
                        labels=["qa", "expert", finding.get('category', 'unknown')],
                        assignee_id=None,
                        qa_source="expert",
                        crate_name=crate_name,
                        issue_type="bug" if finding.get('category') == 'security' else "improvement"
                    )

                    await self._create_linear_issue(linear_issue)
                    issues_created += 1

        logger.info("Linear issues created", crate_name=crate_name, count=issues_created)

    async def _create_pr_tasks_from_qa(self, crate_name: str, lightning_results: Dict, expert_results: Dict):
        """Create PR automation tasks for Claude meta-agents"""
        tasks_created = 0

        # Process PR candidates from Lightning QA
        if lightning_results and "pr_candidates" in lightning_results:
            for pr_candidate in lightning_results["pr_candidates"]:
                pr_task = PRAutomation(
                    title=pr_candidate.get("title", "Lightning QA Improvement"),
                    description=f"""
### Lightning QA PR Suggestion
**Crate**: {crate_name}
**Priority**: {pr_candidate.get('priority', 'medium')}
**Estimated Effort**: {pr_candidate.get('estimated_effort', 'unknown')}

### Suggested Changes
{pr_candidate.get('description', 'No description provided')}

### Context
- Overall Grade: {lightning_results.get('overall_grade', 'N/A')}
- Maintainability Score: {lightning_results.get('maintainability_score', 'N/A')}/10
                    """.strip(),
                    branch_name=f"qa-lightning/{crate_name}/{pr_candidate.get('title', 'fix').lower().replace(' ', '-')}",
                    files_to_modify=[f"src/{crate_name}/*"],
                    suggested_changes=[pr_candidate],
                    priority=pr_candidate.get('priority', 'medium'),
                    estimated_effort=pr_candidate.get('estimated_effort', '2 hours'),
                    claude_agent_type="code_review"
                )

                await self._create_pr_task(pr_task)
                tasks_created += 1

        # Process PR suggestions from Expert QA
        if expert_results and "pr_automation_suggestions" in expert_results:
            for pr_suggestion in expert_results["pr_automation_suggestions"]:
                pr_task = PRAutomation(
                    title=pr_suggestion.get("title", "Expert QA Enhancement"),
                    description=f"""
### Expert QA PR Suggestion
**Crate**: {crate_name}
**Priority**: {pr_suggestion.get('priority', 'medium')}
**AI Generated**: {pr_suggestion.get('ai_generated', False)}
**Estimated Impact**: {pr_suggestion.get('estimated_impact', 'unknown')}

### Enhancement Details
{pr_suggestion.get('description', 'No description provided')}

### Expert Analysis Context
- Platform Expertise Score: {expert_results.get('platform_expertise_score', 'N/A')}/10
- ZenCoder Assessment: {expert_results.get('zencoder_assessment', {}).get('code_quality_score', 'N/A')}/10
                    """.strip(),
                    branch_name=f"qa-expert/{crate_name}/{pr_suggestion.get('title', 'enhancement').lower().replace(' ', '-')}",
                    files_to_modify=[f"src/{crate_name}/*"],
                    suggested_changes=[pr_suggestion],
                    priority=pr_suggestion.get('priority', 'medium'),
                    estimated_effort="3 hours",
                    claude_agent_type="documentation" if "documentation" in pr_suggestion.get('title', '') else "code_review"
                )

                await self._create_pr_task(pr_task)
                tasks_created += 1

        logger.info("PR automation tasks created", crate_name=crate_name, count=tasks_created)

    async def _create_linear_issue(self, issue: LinearIssue) -> str:
        """Create a Linear issue via GraphQL API"""
        if not self.linear_api_key:
            logger.warning("No Linear API key, skipping issue creation")
            return "mock_issue_id"

        query = """
        mutation IssueCreate($input: IssueCreateInput!) {
          issueCreate(input: $input) {
            success
            issue {
              id
              title
              url
            }
          }
        }
        """

        variables = {
            "input": {
                "title": issue.title,
                "description": issue.description,
                "priority": issue.priority,
                "teamId": issue.team_id,
                "labelIds": []  # Would need to map labels to IDs
            }
        }

        try:
            async with httpx.AsyncClient() as client:
                response = await client.post(
                    "https://api.linear.app/graphql",
                    headers={
                        "Content-Type": "application/json",
                        "Authorization": self.linear_api_key,
                    },
                    json={"query": query, "variables": variables}
                )

                if response.status_code == 200:
                    data = response.json()
                    if data.get("data", {}).get("issueCreate", {}).get("success"):
                        issue_id = data["data"]["issueCreate"]["issue"]["id"]
                        logger.info("Linear issue created", issue_id=issue_id, title=issue.title)
                        return issue_id

        except Exception as e:
            logger.error("Failed to create Linear issue", error=str(e))

        return "failed_to_create"

    async def _create_pr_task(self, pr_task: PRAutomation) -> str:
        """Create PR automation task for Claude meta-agents"""
        task_id = f"pr_{pr_task.claude_agent_type}_{int(datetime.utcnow().timestamp())}"

        # Save PR task for Claude meta-agents to pick up
        task_data = asdict(pr_task)
        task_data["task_id"] = task_id
        task_data["status"] = "pending"
        task_data["created_at"] = datetime.utcnow().isoformat()

        # Save to tasks directory for daemon pickup
        os.makedirs("/opt/abe-qa/pr-tasks", exist_ok=True)
        with open(f"/opt/abe-qa/pr-tasks/{task_id}.json", "w") as f:
            json.dump(task_data, f, indent=2)

        logger.info("PR automation task created", task_id=task_id, agent_type=pr_task.claude_agent_type)
        return task_id

    async def _get_linear_teams(self) -> List[Dict]:
        """Get Linear teams"""
        if not self.linear_api_key:
            return [{"id": "mock", "name": "Mock Team", "key": "MOCK"}]

        query = """
        query {
          teams {
            nodes {
              id
              name
              key
              description
            }
          }
        }
        """

        try:
            async with httpx.AsyncClient() as client:
                response = await client.post(
                    "https://api.linear.app/graphql",
                    headers={
                        "Content-Type": "application/json",
                        "Authorization": self.linear_api_key,
                    },
                    json={"query": query}
                )

                if response.status_code == 200:
                    data = response.json()
                    return data.get("data", {}).get("teams", {}).get("nodes", [])

        except Exception as e:
            logger.error("Failed to get Linear teams", error=str(e))

        return []

def main():
    """Main entry point"""
    daemon = LinearQADaemon()

    # Configure uvicorn
    import uvicorn
    config = uvicorn.Config(
        app=daemon.app,
        host="0.0.0.0",
        port=18111,
        log_level="info",
        access_log=True
    )

    server = uvicorn.Server(config)

    logger.info("Starting ABE Linear QA Integration Daemon on port 18111")
    asyncio.run(server.serve())

if __name__ == "__main__":
    main()