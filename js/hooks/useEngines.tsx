import React from "react";
import { WasmEngine } from "../stores/WasmEngine";

const globalContext = React.createContext({
  wasmEngine: new WasmEngine(),
});

export const useEngines = () => React.useContext(globalContext);
