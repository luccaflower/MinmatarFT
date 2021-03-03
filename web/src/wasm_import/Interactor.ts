import {getWasmInstance, WasmInstance} from "./instance";
import {cacheCheck} from "./cache";

export class Interactor {
    private constructor(public wasm: WasmInstance) {

    }
    public static async new(): Promise<Interactor> {
        let wasm = await getWasmInstance();
        return new Interactor(wasm)
    }
    public get ship_names(): string[] {
        return cacheCheck("all_ship_names", () => JSON.parse(this.wasm.all_ship_names()))
    }
}
