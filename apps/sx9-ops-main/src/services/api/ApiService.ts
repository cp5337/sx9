import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';

export interface ApiConfig {
  baseURL: string;
  timeout?: number;
  headers?: Record<string, string>;
  withCredentials?: boolean;
}

export interface ApiRequest {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';
  url: string;
  data?: unknown;
  params?: Record<string, any>;
  headers?: Record<string, string>;
}

export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  status?: number;
}

export class ApiService {
  private client: AxiosInstance;
  private config: ApiConfig;

  constructor(config: ApiConfig) {
    this.config = config;
    this.client = axios.create({
      baseURL: config.baseURL,
      timeout: config.timeout || 10000,
      ...(config.headers && { headers: config.headers }),
      ...(config.withCredentials !== undefined && { withCredentials: config.withCredentials })
    });

    // Request interceptor
    this.client.interceptors.request.use(
      (config) => {
        // Add auth token if available
        const token = localStorage.getItem('authToken');
        if (token) {
          config.headers.Authorization = `Bearer ${token}`;
        }
        return config;
      },
      (error) => Promise.reject(error)
    );

    // Response interceptor
    this.client.interceptors.response.use(
      (response) => response,
      (error) => {
        // Handle common errors
        if (error.response?.status === 401) {
          // Handle unauthorized
          localStorage.removeItem('authToken');
          window.location.href = '/login';
        }
        return Promise.reject(error);
      }
    );
  }

  async request<T = any>(request: ApiRequest): Promise<ApiResponse<T>> {
    try {
      const config: AxiosRequestConfig = {
        method: request.method,
        url: request.url,
        data: request.data,
        ...(request.params && { params: request.params }),
        ...(request.headers && { headers: request.headers })
      };

      const response: AxiosResponse<T> = await this.client.request(config);

      return {
        success: true,
        data: response.data,
        status: response.status
      };
    } catch (error: unknown) {
      const axiosError = error as any;
      return {
        success: false,
        error: axiosError.response?.data?.message || axiosError.message || 'Unknown error',
        status: axiosError.response?.status
      };
    }
  }

  async get<T = any>(url: string, params?: Record<string, any>): Promise<ApiResponse<T>> {
    return this.request<T>({
      method: 'GET',
      url,
      ...(params && { params })
    });
  }

  async post<T = any>(url: string, data?: unknown): Promise<ApiResponse<T>> {
    return this.request<T>({
      method: 'POST',
      url,
      data
    });
  }

  async put<T = any>(url: string, data?: unknown): Promise<ApiResponse<T>> {
    return this.request<T>({
      method: 'PUT',
      url,
      data
    });
  }

  async delete<T = any>(url: string): Promise<ApiResponse<T>> {
    return this.request<T>({
      method: 'DELETE',
      url
    });
  }

  async patch<T = any>(url: string, data?: unknown): Promise<ApiResponse<T>> {
    return this.request<T>({
      method: 'PATCH',
      url,
      data
    });
  }
}

// Default API service instance
export const apiService = new ApiService({
  baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000/api',
  timeout: 10000
});

