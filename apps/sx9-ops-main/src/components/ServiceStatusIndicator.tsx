import React from "react";

interface ServiceStatusIndicatorProps {
  status: "connected" | "running" | "disconnected" | "stopped" | "error" | "warning";
  pulse?: boolean;
  size?: "sm" | "md" | "lg";
  className?: string;
}

export function ServiceStatusIndicator({
  status,
  pulse = false,
  size = "md",
  className = "",
}: ServiceStatusIndicatorProps) {
  const sizeClasses = {
    sm: "w-2 h-2",
    md: "w-3 h-3",
    lg: "w-4 h-4",
  };

  const colorClasses = {
    connected: "bg-status-connected",
    running: "bg-status-running",
    disconnected: "bg-status-disconnected",
    stopped: "bg-status-stopped",
    error: "bg-status-error",
    warning: "bg-status-warning",
  };

  return (
    <div
      className={`rounded-full ${sizeClasses[size]} ${colorClasses[status]} ${
        pulse ? "animate-pulse-slow" : ""
      } ${className}`}
      title={status.charAt(0).toUpperCase() + status.slice(1)}
    />
  );
}
