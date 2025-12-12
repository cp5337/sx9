#!/usr/bin/env python3
"""
CTAS-7 Kali Tools Task Server v7.3.1
Exposes 165 CTAS tasks via REST API
"""

from flask import Flask, jsonify, request
import subprocess
import json
import os

app = Flask(__name__)

# Load CTAS tasks
TASKS_FILE = os.path.join(os.path.dirname(__file__), 'tasks.json')

@app.route('/health', methods=['GET'])
def health():
    return jsonify({
        'status': 'healthy',
        'service': 'ctas7-kali-tools',
        'version': '7.3.1'
    })

@app.route('/api/tasks', methods=['GET'])
def list_tasks():
    """List all 165 CTAS tasks"""
    try:
        with open(TASKS_FILE, 'r') as f:
            tasks = json.load(f)
        return jsonify({
            'tasks': tasks,
            'count': len(tasks)
        })
    except Exception as e:
        return jsonify({'error': str(e)}), 500

@app.route('/api/tasks/<task_id>/execute', methods=['POST'])
def execute_task(task_id):
    """Execute a CTAS task with escalation ladder"""
    data = request.get_json() or {}
    level = data.get('level', 'script')  # script, microkernel, binary, container
    
    app.logger.info(f"Executing task {task_id} at level {level}")
    
    # Mock execution for now
    return jsonify({
        'task_id': task_id,
        'level': level,
        'status': 'executed',
        'result': f'Task {task_id} executed at {level} level'
    })

@app.route('/api/tools/nmap', methods=['POST'])
def run_nmap():
    """Run nmap scan"""
    data = request.get_json() or {}
    target = data.get('target', '127.0.0.1')
    
    try:
        result = subprocess.run(
            ['nmap', '-sn', target],
            capture_output=True,
            text=True,
            timeout=30
        )
        return jsonify({
            'tool': 'nmap',
            'target': target,
            'output': result.stdout,
            'returncode': result.returncode
        })
    except Exception as e:
        return jsonify({'error': str(e)}), 500

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=15178, debug=False)

