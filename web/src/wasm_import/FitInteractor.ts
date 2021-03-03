import {Interactor} from "./Interactor";
import {WasmInstance} from "./instance";
import {cacheCheck} from "./cache";
import {CompressedFit} from "../types/CompressedFit";

export class FitInteractor {
    private finalizationRegistry: FinalizationRegistry
    private readonly finalizationRegistryToken: object
    private constructor(public wasm: WasmInstance, private ship_pointer: BigInt) {
        this.finalizationRegistry = cacheCheck("fit_finalization_registry", () => new FinalizationRegistry(wasm.drop_fit))
        this.finalizationRegistryToken = {}
        this.finalizationRegistry.register(this, ship_pointer, this.finalizationRegistryToken)
    }

    public drop() {
        this.finalizationRegistry.unregister(this.finalizationRegistryToken)
        this.wasm.drop_fit(this.ship_pointer)
    }

    public static new(_wasm: WasmInstance | Interactor, shipName: string): FitInteractor | null {
        let wasm: WasmInstance;
        if (_wasm instanceof Interactor) {
            wasm = _wasm.wasm
        }
        let pointer = wasm.new_fit(shipName);
        if (pointer == undefined) {
            return null
        }
        return new FitInteractor(wasm, pointer);
    }

    public set name(value: string) {
        this.wasm.rename_fit(this.ship_pointer, value)
    }

    public get name(): string {
        return this.wasm.get_name_fit(this.ship_pointer)
    }

    public save() {
        let str = this.wasm.compress_fit(this.ship_pointer);
        let compressed: CompressedFit = JSON.parse(str);
        console.log(compressed)
        localStorage.setItem("fit_" + compressed.name, JSON.stringify(compressed))
    }
}
