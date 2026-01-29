import React, { useEffect, useRef } from 'react';
import { OpenSheetMusicDisplay } from 'opensheetmusicdisplay';

interface ScoreDisplayProps {
    xml: string;
}

const ScoreDisplay: React.FC<ScoreDisplayProps> = ({ xml }) => {
    const containerRef = useRef<HTMLDivElement>(null);
    const osmdRef = useRef<OpenSheetMusicDisplay | null>(null);

    useEffect(() => {
        if (containerRef.current && !osmdRef.current) {
            osmdRef.current = new OpenSheetMusicDisplay(containerRef.current, {
                autoResize: true,
                drawTitle: true,
            });
        }
    }, []);

    useEffect(() => {
        const render = async () => {
            if (osmdRef.current && xml) {
                try {
                    await osmdRef.current.load(xml);
                    osmdRef.current.render();
                } catch (e) {
                    console.error("OSMD Error:", e);
                }
            }
        };
        render();
    }, [xml]);

    return <div ref={containerRef} style={{ width: '100%', height: '100%', overflow: 'auto' }} />;
};

export default ScoreDisplay;
