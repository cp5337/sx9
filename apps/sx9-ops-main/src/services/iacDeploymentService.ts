/**
 * IAC Deployment Service
 * Handles Terraform deployment for CTAS infrastructure
 */

export interface IACDeploymentRequest {
  moduleId: string;
  modulePath: string;
  config: {
    projectId: string;
    region: string;
    environment: 'dev' | 'staging' | 'prod';
  };
}

export interface IACDeploymentResponse {
  success: boolean;
  moduleId: string;
  message: string;
  outputs?: Record<string, any>;
  logs?: string[];
}

export class IACDeploymentService {
  private baseUrl = '/api/iac';

  async deployModule(request: IACDeploymentRequest): Promise<IACDeploymentResponse> {
    try {
      const response = await fetch(`${this.baseUrl}/deploy`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(request)
      });

      if (!response.ok) {
        throw new Error(`Deployment failed: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      return {
        success: false,
        moduleId: request.moduleId,
        message: error instanceof Error ? error.message : 'Unknown error'
      };
    }
  }

  async getDeploymentStatus(moduleId: string): Promise<any> {
    try {
      const response = await fetch(`${this.baseUrl}/status/${moduleId}`);
      return await response.json();
    } catch (error) {
      return { status: 'unknown', error: error instanceof Error ? error.message : 'Unknown error' };
    }
  }

  async listDeployments(): Promise<any[]> {
    try {
      const response = await fetch(`${this.baseUrl}/list`);
      return await response.json();
    } catch (error) {
      return [];
    }
  }
}

export const iacDeploymentService = new IACDeploymentService();



