/*
 * GLAF Performance Dashboard
 *
 * Displays real-time metrics from GLAF architecture:
 * - Legion Hot Path (query latency, throughput)
 * - Ring Buffer (utilization, head/tail positions)
 * - APECS Layer (event processing, errors)
 * - Bridge (workflow execution stats)
 * - Crystal Tuning (precision, speed, depth, noise)
 */

import React, { useState, useEffect } from 'react';
import { Activity, Zap, Database, GitBranch, Settings, TrendingUp } from 'lucide-react';
import { SynaptixGLAFBridge } from '../lib/glaf/synaptixBridge';

interface GLAFDashboardProps {
  bridge: SynaptixGLAFBridge;
  refreshInterval?: number;
}

export function GLAFDashboard({ bridge, refreshInterval = 1000 }: GLAFDashboardProps) {
  const [legionMetrics, setLegionMetrics] = useState<ReturnType<typeof bridge.getLegionMetrics>>();
  const [apecsMetrics, setAPECSMetrics] = useState<ReturnType<typeof bridge.getAPECSMetrics>>();
  const [bridgeMetrics, setBridgeMetrics] = useState<ReturnType<typeof bridge.getMetrics>>();
  const [crystalTuning, setCrystalTuning] = useState(bridge.getCrystalTuning());

  useEffect(() => {
    const updateMetrics = () => {
      setLegionMetrics(bridge.getLegionMetrics());
      setAPECSMetrics(bridge.getAPECSMetrics());
      setBridgeMetrics(bridge.getMetrics());
    };

    updateMetrics();
    const interval = setInterval(updateMetrics, refreshInterval);
    return () => clearInterval(interval);
  }, [bridge, refreshInterval]);

  const handleCrystalChange = (key: keyof typeof crystalTuning, value: number) => {
    const updated = { ...crystalTuning, [key]: value };
    setCrystalTuning(updated);
    bridge.setCrystalTuning({ [key]: value });
  };

  const formatLatency = (ns: number) => {
    if (ns < 1000) return `${ns.toFixed(0)}ns`;
    if (ns < 1000000) return `${(ns / 1000).toFixed(1)}µs`;
    return `${(ns / 1000000).toFixed(2)}ms`;
  };

  const formatNumber = (n: bigint | number) => {
    return typeof n === 'bigint' ? n.toString() : n.toFixed(0);
  };

  return (
    <div className="space-y-6 p-6 bg-neutral-900 text-neutral-100">
      <div className="flex items-center gap-3 mb-4">
        <Zap className="w-8 h-8 text-blue-400" />
        <h2 className="text-2xl font-bold">GLAF Performance Monitor</h2>
      </div>

      {/* Legion Hot Path Metrics */}
      <div className="bg-neutral-800 rounded-lg p-6 border border-neutral-700">
        <div className="flex items-center gap-2 mb-4">
          <Zap className="w-5 h-5 text-yellow-400" />
          <h3 className="text-lg font-semibold">Legion Hot Path</h3>
          <span className="ml-auto text-xs text-neutral-400">
            Target: &lt;1µs deterministic
          </span>
        </div>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <MetricCard
            label="Query Count"
            value={formatNumber(legionMetrics?.queryCount || 0n)}
            icon={<TrendingUp className="w-4 h-4" />}
          />
          <MetricCard
            label="Entity Count"
            value={formatNumber(legionMetrics?.entityCount || 0)}
            icon={<Database className="w-4 h-4" />}
          />
          <MetricCard
            label="Ring Buffer Usage"
            value={`${legionMetrics?.ringBufferMetrics.utilization.toFixed(1) || 0}%`}
            icon={<Activity className="w-4 h-4" />}
            warning={
              (legionMetrics?.ringBufferMetrics.utilization || 0) > 80
            }
          />
          <MetricCard
            label="Buffer Available"
            value={formatNumber(legionMetrics?.ringBufferMetrics.available || 0)}
            icon={<Database className="w-4 h-4" />}
          />
        </div>
      </div>

      {/* Ring Buffer Metrics */}
      <div className="bg-neutral-800 rounded-lg p-6 border border-neutral-700">
        <div className="flex items-center gap-2 mb-4">
          <Activity className="w-5 h-5 text-green-400" />
          <h3 className="text-lg font-semibold">SPSC Ring Buffer</h3>
          <span className="ml-auto text-xs text-neutral-400">
            Write: 30ns | Read: 50ns
          </span>
        </div>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <MetricCard
            label="Capacity"
            value={formatNumber(legionMetrics?.ringBufferMetrics.capacity || 0)}
            icon={<Database className="w-4 h-4" />}
          />
          <MetricCard
            label="Head Position"
            value={formatNumber(legionMetrics?.ringBufferMetrics.head || 0n)}
            icon={<TrendingUp className="w-4 h-4" />}
          />
          <MetricCard
            label="Tail Position"
            value={formatNumber(legionMetrics?.ringBufferMetrics.tail || 0n)}
            icon={<TrendingUp className="w-4 h-4" />}
          />
          <MetricCard
            label="Remaining Slots"
            value={formatNumber(legionMetrics?.ringBufferMetrics.remaining || 0)}
            icon={<Activity className="w-4 h-4" />}
          />
        </div>

        {/* Utilization Bar */}
        <div className="mt-4">
          <div className="flex justify-between text-xs text-neutral-400 mb-1">
            <span>Buffer Utilization</span>
            <span>{legionMetrics?.ringBufferMetrics.utilization.toFixed(2)}%</span>
          </div>
          <div className="w-full h-2 bg-neutral-700 rounded-full overflow-hidden">
            <div
              className={`h-full transition-all duration-300 ${
                (legionMetrics?.ringBufferMetrics.utilization || 0) > 80
                  ? 'bg-red-500'
                  : (legionMetrics?.ringBufferMetrics.utilization || 0) > 50
                  ? 'bg-yellow-500'
                  : 'bg-green-500'
              }`}
              style={{
                width: `${legionMetrics?.ringBufferMetrics.utilization || 0}%`,
              }}
            />
          </div>
        </div>
      </div>

      {/* APECS Async Layer Metrics */}
      <div className="bg-neutral-800 rounded-lg p-6 border border-neutral-700">
        <div className="flex items-center gap-2 mb-4">
          <GitBranch className="w-5 h-5 text-purple-400" />
          <h3 className="text-lg font-semibold">APECS Async Layer</h3>
          <span className="ml-auto text-xs text-neutral-400">
            Poll: 1ms interval
          </span>
        </div>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <MetricCard
            label="Events Processed"
            value={formatNumber(apecsMetrics?.eventsProcessed || 0n)}
            icon={<Activity className="w-4 h-4" />}
          />
          <MetricCard
            label="Events/Second"
            value={formatNumber(apecsMetrics?.eventsPerSecond || 0)}
            icon={<TrendingUp className="w-4 h-4" />}
          />
          <MetricCard
            label="Avg Latency"
            value={`${apecsMetrics?.avgLatencyMs.toFixed(2) || 0}ms`}
            icon={<Zap className="w-4 h-4" />}
          />
          <MetricCard
            label="Error Count"
            value={formatNumber(apecsMetrics?.errorCount || 0n)}
            icon={<Activity className="w-4 h-4" />}
            warning={(apecsMetrics?.errorCount || 0n) > 0n}
          />
        </div>
      </div>

      {/* Bridge Metrics */}
      <div className="bg-neutral-800 rounded-lg p-6 border border-neutral-700">
        <div className="flex items-center gap-2 mb-4">
          <GitBranch className="w-5 h-5 text-cyan-400" />
          <h3 className="text-lg font-semibold">SYNAPTIX9 Bridge</h3>
        </div>

        <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
          <MetricCard
            label="Total Executions"
            value={formatNumber(bridgeMetrics?.totalExecutions || 0n)}
            icon={<Activity className="w-4 h-4" />}
          />
          <MetricCard
            label="Successful"
            value={formatNumber(bridgeMetrics?.successfulExecutions || 0n)}
            icon={<TrendingUp className="w-4 h-4 text-green-400" />}
          />
          <MetricCard
            label="Failed"
            value={formatNumber(bridgeMetrics?.failedExecutions || 0n)}
            icon={<Activity className="w-4 h-4 text-red-400" />}
            warning={(bridgeMetrics?.failedExecutions || 0n) > 0n}
          />
          <MetricCard
            label="Avg Execution"
            value={`${bridgeMetrics?.avgExecutionTimeMs.toFixed(2) || 0}ms`}
            icon={<Zap className="w-4 h-4" />}
          />
          <MetricCard
            label="Hot Path Latency"
            value={formatLatency(bridgeMetrics?.hotPathLatencyNs || 0)}
            icon={<Zap className="w-4 h-4" />}
          />
        </div>
      </div>

      {/* Crystal Tuning */}
      <div className="bg-neutral-800 rounded-lg p-6 border border-neutral-700">
        <div className="flex items-center gap-2 mb-4">
          <Settings className="w-5 h-5 text-orange-400" />
          <h3 className="text-lg font-semibold">Crystal Tuning</h3>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <CrystalSlider
            label="Precision"
            value={crystalTuning.precision}
            onChange={(v) => handleCrystalChange('precision', v)}
            color="blue"
          />
          <CrystalSlider
            label="Speed"
            value={crystalTuning.speed}
            onChange={(v) => handleCrystalChange('speed', v)}
            color="green"
          />
          <CrystalSlider
            label="Depth"
            value={crystalTuning.depth}
            onChange={(v) => handleCrystalChange('depth', v)}
            color="purple"
          />
          <CrystalSlider
            label="Noise"
            value={crystalTuning.noise}
            onChange={(v) => handleCrystalChange('noise', v)}
            color="orange"
          />
        </div>
      </div>
    </div>
  );
}

interface MetricCardProps {
  label: string;
  value: string;
  icon: React.ReactNode;
  warning?: boolean;
}

function MetricCard({ label, value, icon, warning }: MetricCardProps) {
  return (
    <div className="bg-neutral-900 rounded p-4 border border-neutral-700">
      <div className="flex items-center gap-2 text-neutral-400 text-xs mb-2">
        {icon}
        <span>{label}</span>
      </div>
      <div
        className={`text-2xl font-bold ${
          warning ? 'text-red-400' : 'text-neutral-100'
        }`}
      >
        {value}
      </div>
    </div>
  );
}

interface CrystalSliderProps {
  label: string;
  value: number;
  onChange: (value: number) => void;
  color: 'blue' | 'green' | 'purple' | 'orange';
}

function CrystalSlider({ label, value, onChange, color }: CrystalSliderProps) {

  return (
    <div>
      <div className="flex justify-between text-sm mb-2">
        <label className="text-neutral-300">{label}</label>
        <span className="text-neutral-400">{value.toFixed(2)}</span>
      </div>
      <input
        type="range"
        min="0"
        max="1"
        step="0.01"
        value={value}
        onChange={(e) => onChange(parseFloat(e.target.value))}
        className="w-full h-2 rounded-full appearance-none cursor-pointer bg-neutral-700"
        style={{
          background: `linear-gradient(to right, ${
            color === 'blue'
              ? '#3b82f6'
              : color === 'green'
              ? '#10b981'
              : color === 'purple'
              ? '#a855f7'
              : '#f97316'
          } 0%, ${
            color === 'blue'
              ? '#3b82f6'
              : color === 'green'
              ? '#10b981'
              : color === 'purple'
              ? '#a855f7'
              : '#f97316'
          } ${value * 100}%, #404040 ${value * 100}%, #404040 100%)`,
        }}
      />
    </div>
  );
}
