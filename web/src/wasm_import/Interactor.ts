import {getWasmInstance, WasmInstance} from "./instance";
import {cacheCheck} from "./cache";
import {Ship} from "../types/Ship";
import {FitInteractor} from "./FitInteractor";

export class Interactor {
    private constructor(public wasm: WasmInstance) {

    }

    public static async new(): Promise<Interactor> {
        let wasm = await getWasmInstance();
        return new Interactor(wasm)
    }

    public get shipNames(): string[] {
        return cacheCheck("all_ship_names", () => this.wasm.all_ship_names())
    }

    public get ships(): Ship[] {
        return cacheCheck("all_ships", () => this.wasm.all_ships())
    }

    public fetchShip(name: string): Ship | null {
        return this.wasm.fetch_ship_by_name(name)
    }

    public newFit(shipName: string, fitName: string | null = null): FitInteractor | null {
        let fit = FitInteractor.new(this, shipName)
        if (fit !== null) {
            if (fitName !== null) {
                fit.name = fitName
            }
            return fit
        } else {
            return null
        }
    }
}
