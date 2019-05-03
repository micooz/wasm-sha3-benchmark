/* tslint:disable */
/**
* @param {string} text 
* @returns {string} 
*/
export function sha3_256(text: string): string;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export function init (module_or_path: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        