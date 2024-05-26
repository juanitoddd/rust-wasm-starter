import React, { useEffect, useRef, useCallback } from "react";
import { useEngines } from "../hooks/useEngines";
import useWindowDimensions from "../hooks/useWindowDimensions";

const styles = {
  container: {
    display: "flex",
    justifyContent: "center",
    margin: "0 auto",
  },
};

export function Greet() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { wasmEngine } = useEngines();

  // Canvas Size
  const { height, width } = useWindowDimensions();

  const handleUserKeyPress = useCallback((event) => {
    const { key, keyCode } = event;
    if (wasmEngine.instance) {
      wasmEngine.instance.keyboard_event(key);
    }
  }, []);

  useEffect(() => {
    window.addEventListener("keydown", handleUserKeyPress);
    return () => {
      window.removeEventListener("keydown", handleUserKeyPress);
    };
  }, [handleUserKeyPress]);

  useEffect(() => {
    let canvas: HTMLCanvasElement;
    if (wasmEngine.instance && canvasRef.current) {
      canvas = canvasRef.current;
      wasmEngine.instance.start_game(canvas, "Alex", "Angelica");
    }
  }, [canvasRef, wasmEngine, width, height]);

  return (
    <div style={styles.container}>
      <canvas ref={canvasRef} width={`${width}`} height={`${height}`}></canvas>
    </div>
  );
}
