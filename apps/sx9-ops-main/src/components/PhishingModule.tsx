import React, { useState } from 'react';
import { Mail, Target, Users, Shield, AlertTriangle, CheckCircle, Clock } from 'lucide-react';

interface PhishingCampaign {
  id: string;
  name: string;
  target: string;
  template: string;
  status: 'draft' | 'active' | 'completed' | 'paused';
  sentCount: number;
  openedCount: number;
  clickedCount: number;
  compromisedCount: number;
  createdAt: string;
  lastSent: string;
}

const PhishingModule: React.FC = () => {
  const [campaigns, setCampaigns] = useState<PhishingCampaign[]>([]);
  const [selectedCampaign, setSelectedCampaign] = useState<PhishingCampaign | null>(null);
  const [isCreating, setIsCreating] = useState(false);

  const phishingTemplates = [
    {
      id: 'password-reset',
      name: 'Password Reset Request',
      subject: 'Urgent: Password Reset Required',
      description: 'Simulates a password reset email from IT department'
    },
    {
      id: 'security-alert',
      name: 'Security Alert',
      subject: 'Security Breach Detected',
      description: 'Simulates a security alert requiring immediate action'
    },
    {
      id: 'invoice-payment',
      name: 'Invoice Payment',
      subject: 'Payment Overdue - Immediate Action Required',
      description: 'Simulates an urgent invoice payment request'
    },
    {
      id: 'system-update',
      name: 'System Update Required',
      subject: 'Critical System Update - Action Required',
      description: 'Simulates a critical system update notification'
    }
  ];

  const createPhishingCampaign = async (campaignData: Partial<PhishingCampaign>) => {
    setIsCreating(true);
    
    try {
      // Production phishing campaign creation
      const campaign = await createCampaign(campaignData);
      setCampaigns(prev => [...prev, campaign]);
      setIsCreating(false);
      return campaign;
    } catch (error) {
      // Fallback to demo data if production fails
      if (import.meta.env.VITE_DEMO_MODE === 'true') {
        const demoCampaign: PhishingCampaign = {
          id: `campaign-${Date.now()}`,
          name: campaignData.name || 'Demo Campaign',
          target: campaignData.target || 'demo@company.com',
          template: campaignData.template || 'password-reset',
          status: 'draft',
          sentCount: 0,
          openedCount: 0,
          clickedCount: 0,
          compromisedCount: 0,
          createdAt: new Date().toISOString(),
          lastSent: new Date().toISOString()
        };
        setCampaigns(prev => [...prev, demoCampaign]);
        setIsCreating(false);
        return demoCampaign;
      } else {
        console.error('Campaign creation failed:', error);
        setIsCreating(false);
        throw error;
      }
    }
  };

  const createCampaign = async (campaignData: Partial<PhishingCampaign>): Promise<PhishingCampaign> => {
    // Production implementation would integrate with Bolt.new or similar services
    // For now, this throws an error to trigger demo fallback
    throw new Error('Phishing service not configured');
  };

  const sendCampaign = async (campaignId: string) => {
    const campaign = campaigns.find(c => c.id === campaignId);
    if (!campaign) return;

    try {
      // Production campaign sending
      await sendPhishingEmails(campaign);
      setCampaigns(prev => prev.map(c => 
        c.id === campaignId 
          ? { ...c, status: 'active', lastSent: new Date().toISOString() }
          : c
      ));
    } catch (error) {
      if (import.meta.env.VITE_DEMO_MODE === 'true') {
        // Demo mode - simulate sending
        setCampaigns(prev => prev.map(c => 
          c.id === campaignId 
            ? { 
                ...c, 
                status: 'active', 
                sentCount: c.sentCount + 100,
                lastSent: new Date().toISOString() 
              }
            : c
        ));
      } else {
        console.error('Campaign sending failed:', error);
      }
    }
  };

  const sendPhishingEmails = async (campaign: PhishingCampaign): Promise<void> => {
    // Production implementation would send actual emails
    throw new Error('Email service not configured');
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'bg-green-100 text-green-800';
      case 'draft':
        return 'bg-gray-100 text-gray-800';
      case 'completed':
        return 'bg-blue-100 text-blue-800';
      case 'paused':
        return 'bg-yellow-100 text-yellow-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'draft':
        return <Clock className="w-4 h-4 text-gray-500" />;
      case 'completed':
        return <Shield className="w-4 h-4 text-blue-500" />;
      case 'paused':
        return <AlertTriangle className="w-4 h-4 text-yellow-500" />;
      default:
        return <Clock className="w-4 h-4 text-gray-500" />;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Header */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <Mail className="w-8 h-8 text-blue-600" />
              <h1 className="text-3xl font-bold text-gray-900">Phishing Simulation</h1>
            </div>
            <div className="flex items-center gap-2">
              <span className="bg-red-100 text-red-800 px-3 py-1 rounded text-sm font-semibold">
                Security Training
              </span>
            </div>
          </div>
          
          <p className="text-gray-600 mb-6">
            Create and manage phishing simulation campaigns to test employee security awareness.
          </p>

          {/* Quick Stats */}
          <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <Target className="w-4 h-4 text-blue-500" />
                <span className="text-sm font-medium text-blue-800">Total Campaigns</span>
              </div>
              <span className="text-2xl font-bold text-blue-900">{campaigns.length}</span>
            </div>
            <div className="bg-green-50 border border-green-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <Users className="w-4 h-4 text-green-500" />
                <span className="text-sm font-medium text-green-800">Emails Sent</span>
              </div>
              <span className="text-2xl font-bold text-green-900">
                {campaigns.reduce((sum, c) => sum + c.sentCount, 0)}
              </span>
            </div>
            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <AlertTriangle className="w-4 h-4 text-yellow-500" />
                <span className="text-sm font-medium text-yellow-800">Clicked Rate</span>
              </div>
              <span className="text-2xl font-bold text-yellow-900">
                {campaigns.length > 0 
                  ? Math.round((campaigns.reduce((sum, c) => sum + c.clickedCount, 0) / 
                     campaigns.reduce((sum, c) => sum + c.sentCount, 1)) * 100)
                  : 0}%
              </span>
            </div>
            <div className="bg-red-50 border border-red-200 rounded-lg p-4">
              <div className="flex items-center gap-2 mb-2">
                <Shield className="w-4 h-4 text-red-500" />
                <span className="text-sm font-medium text-red-800">Compromised</span>
              </div>
              <span className="text-2xl font-bold text-red-900">
                {campaigns.reduce((sum, c) => sum + c.compromisedCount, 0)}
              </span>
            </div>
          </div>
        </div>

        {/* Campaign Templates */}
        <div className="bg-white rounded-lg shadow-xl p-6 mb-6">
          <h2 className="text-2xl font-semibold text-gray-800 mb-6">Phishing Templates</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
            {phishingTemplates.map((template) => (
              <div key={template.id} className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
                <h3 className="font-semibold text-gray-900 mb-2">{template.name}</h3>
                <p className="text-sm text-gray-600 mb-3">{template.description}</p>
                <div className="text-xs text-gray-500 mb-3">
                  <strong>Subject:</strong> {template.subject}
                </div>
                <button
                  onClick={() => {
                    const campaign = createPhishingCampaign({
                      name: `${template.name} Campaign`,
                      template: template.id,
                      target: 'employees@company.com'
                    });
                  }}
                  disabled={isCreating}
                  className="w-full px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors text-sm"
                >
                  {isCreating ? 'Creating...' : 'Use Template'}
                </button>
              </div>
            ))}
          </div>
        </div>

        {/* Campaigns List */}
        {campaigns.length > 0 && (
          <div className="bg-white rounded-lg shadow-xl p-6">
            <h2 className="text-2xl font-semibold text-gray-800 mb-6">Active Campaigns</h2>
            
            <div className="space-y-4">
              {campaigns.map((campaign) => (
                <div key={campaign.id} className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex items-center gap-3">
                      {getStatusIcon(campaign.status)}
                      <div>
                        <h3 className="font-semibold text-gray-900">{campaign.name}</h3>
                        <span className={`inline-block px-2 py-1 rounded text-xs font-semibold mt-1 ${getStatusColor(campaign.status)}`}>
                          {campaign.status}
                        </span>
                      </div>
                    </div>
                    <div className="flex gap-2">
                      {campaign.status === 'draft' && (
                        <button
                          onClick={() => sendCampaign(campaign.id)}
                          className="px-3 py-1 bg-green-600 text-white rounded text-sm hover:bg-green-700 transition-colors"
                        >
                          Send
                        </button>
                      )}
                      <button
                        onClick={() => setSelectedCampaign(campaign)}
                        className="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700 transition-colors"
                      >
                        View Details
                      </button>
                    </div>
                  </div>
                  
                  <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                    <div>
                      <span className="text-gray-500">Target:</span>
                      <p className="font-medium">{campaign.target}</p>
                    </div>
                    <div>
                      <span className="text-gray-500">Sent:</span>
                      <p className="font-medium">{campaign.sentCount}</p>
                    </div>
                    <div>
                      <span className="text-gray-500">Opened:</span>
                      <p className="font-medium">{campaign.openedCount}</p>
                    </div>
                    <div>
                      <span className="text-gray-500">Clicked:</span>
                      <p className="font-medium">{campaign.clickedCount}</p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Campaign Detail Modal */}
        {selectedCampaign && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-2xl w-full max-h-[80vh] overflow-y-auto">
              <div className="p-6">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <Mail className="w-6 h-6 text-blue-600" />
                    <h2 className="text-2xl font-bold text-gray-900">
                      {selectedCampaign.name}
                    </h2>
                  </div>
                  <button
                    onClick={() => setSelectedCampaign(null)}
                    className="text-gray-400 hover:text-gray-600"
                  >
                    <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                  </button>
                </div>
                
                <div className="space-y-4">
                  <div className="grid grid-cols-2 gap-4">
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Status</h3>
                      <div className="flex items-center gap-2 mt-1">
                        {getStatusIcon(selectedCampaign.status)}
                        <span className="capitalize">{selectedCampaign.status}</span>
                      </div>
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Template</h3>
                      <p className="mt-1 capitalize">{selectedCampaign.template}</p>
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Target</h3>
                      <p className="mt-1">{selectedCampaign.target}</p>
                    </div>
                    <div>
                      <h3 className="text-sm font-medium text-gray-500">Created</h3>
                      <p className="mt-1">{new Date(selectedCampaign.createdAt).toLocaleDateString()}</p>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-gray-500 mb-2">Campaign Metrics</h3>
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                      <div className="bg-blue-50 p-3 rounded">
                        <div className="text-2xl font-bold text-blue-900">{selectedCampaign.sentCount}</div>
                        <div className="text-sm text-blue-600">Sent</div>
                      </div>
                      <div className="bg-green-50 p-3 rounded">
                        <div className="text-2xl font-bold text-green-900">{selectedCampaign.openedCount}</div>
                        <div className="text-sm text-green-600">Opened</div>
                      </div>
                      <div className="bg-yellow-50 p-3 rounded">
                        <div className="text-2xl font-bold text-yellow-900">{selectedCampaign.clickedCount}</div>
                        <div className="text-sm text-yellow-600">Clicked</div>
                      </div>
                      <div className="bg-red-50 p-3 rounded">
                        <div className="text-2xl font-bold text-red-900">{selectedCampaign.compromisedCount}</div>
                        <div className="text-sm text-red-600">Compromised</div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default PhishingModule;