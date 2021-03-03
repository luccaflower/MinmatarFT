import {getWasmInstance, WasmInstance} from "./instance";
import {cacheCheck} from "./cache";
import {Ship} from "../types/ships";

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

    public get ships(): Ship[] {
        return cacheCheck("all_ships", () => JSON.parse(this.wasm.all_ships()))
    }

    public fetch_ship(name: string): Ship | null {
        let ship = this.wasm.fetch_ship_by_name(name)
        if (!ship) {
            return null
        }
        return JSON.parse(ship)
    }
}
