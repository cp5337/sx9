import React, { createContext, useContext, useState } from 'react';
import { useNavigate } from 'react-router-dom';

interface User {
  id: string;
  username: string;
  role: 'admin' | 'operator' | 'analyst' | 'viewer';
  permissions: string[];
}

interface AuthContextType {
  user: User | null;
  loading: boolean;
  error: string | null;
  login: (username: string, password: string) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(() => {
    // Auto-login for development
    const demoUser = {
      id: '1',
      username: 'admin',
      role: 'admin' as const,
      permissions: ['read', 'write', 'execute']
    };
    return demoUser;
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const navigate = useNavigate();

  const login = async (username: string, password: string) => {
    try {
      setLoading(true);
      setError(null);

      // Demo authentication - replace with real auth in production
      if (username === 'admin' && password === 'admin') {
        const user = {
          id: '1',
          username: 'admin',
          role: 'admin' as const,
          permissions: ['read', 'write', 'execute']
        };
        localStorage.setItem('auth_token', 'demo_token');
        setUser(user);
        navigate('/');
      } else {
        throw new Error('Invalid credentials');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Login failed');
      throw err;
    } finally {
      setLoading(false);
    }
  };

  const logout = () => {
    localStorage.removeItem('auth_token');
    setUser(null);
    navigate('/login');
  };

  return (
    <AuthContext.Provider value={{ user, loading, error, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};