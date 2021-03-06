import {Interactor} from "./src/wasm_import/Interactor";
import {FitInteractor} from "./src/wasm_import/FitInteractor";

(async ()=> {
    let interactor = await Interactor.new();
    console.log(interactor.shipNames)
    console.log(JSON.stringify(interactor.ships))
    let fit_interactor = interactor.newFit(interactor.shipNames[0])
    fit_interactor.name = "cool name"
    console.log(fit_interactor.name)
    fit_interactor.save()
})()
