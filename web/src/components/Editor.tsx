import React, { useEffect, useRef } from 'react';
import { EditorState } from '@codemirror/state';
import { EditorView, keymap } from '@codemirror/view';
import { defaultKeymap } from '@codemirror/commands';
import { rust } from '@codemirror/lang-rust';

interface EditorProps {
    code: string;
    onChange: (value: string) => void;
}

const Editor: React.FC<EditorProps> = ({ code, onChange }) => {
    const editorRef = useRef<HTMLDivElement>(null);
    const viewRef = useRef<EditorView | null>(null);

    useEffect(() => {
        if (!editorRef.current) return;

        const state = EditorState.create({
            doc: code,
            extensions: [
                keymap.of(defaultKeymap),
                rust(), // Using rust highlighting as it's somewhat similar to Melos
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        onChange(update.state.doc.toString());
                    }
                }),
            ],
        });

        const view = new EditorView({
            state,
            parent: editorRef.current,
        });

        viewRef.current = view;

        return () => {
            view.destroy();
        };
    }, []);

    useEffect(() => {
        const view = viewRef.current;
        if (!view) return;
        const current = view.state.doc.toString();
        if (current === code) return;
        view.dispatch({
            changes: { from: 0, to: current.length, insert: code },
        });
    }, [code]);

    return <div ref={editorRef} style={{ height: '100%', width: '100%', textAlign: 'left' }} />;
};

export default Editor;
