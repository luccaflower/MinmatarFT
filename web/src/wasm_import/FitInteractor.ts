import {Interactor} from "./Interactor";
import {WasmInstance} from "./instance";

export class FitInteractor {
    private constructor(public wasm: WasmInstance, private ship_pointer: BigInt) {

    }

    public static new(wasm: WasmInstance | Interactor, shipName: string): FitInteractor | null {
        if (wasm instanceof Interactor) {
            wasm = wasm.wasm
        }
        let pointer = wasm.new_fit(shipName);
        if(pointer == undefined) {
            return null
        }
        let interactor = new FitInteractor(wasm, pointer);
        new FinalizationRegistry(wasm.drop_fit).register(interactor, pointer)
        return interactor
    }

    public set name(value: string) {
        this.wasm.rename_fit(this.ship_pointer, value)
    }

    public get name(): string {
        return this.wasm.get_name_fit(this.ship_pointer)
    }
}
