import React from 'react';

interface DebugPanelProps {
  request: string;
  response: string;
  error: string;
}

const DebugPanel: React.FC<DebugPanelProps> = ({ request, response, error }) => {
  return (
    <div className="debug-panel">
      <div className="debug-section">
        <div className="debug-title">Last request</div>
        <pre>{request || 'No requests yet.'}</pre>
      </div>
      <div className="debug-section">
        <div className="debug-title">Last response</div>
        <pre>{response || error || 'No responses yet.'}</pre>
      </div>
    </div>
  );
};

export default DebugPanel;
