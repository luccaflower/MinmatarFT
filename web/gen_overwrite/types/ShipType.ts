import {FrigateType} from "./FrigateType";
import {CruiserType} from "./CruiserType";
import {BattlecruiserType} from "./BattlecruiserType";
import {BattleshipType} from "./BattleshipType";
import {DestroyerType} from "./DestroyerType";

export type ShipType = ["Frigate" | "Destroyer" | "Cruiser" | "Battlecruiser" | "Battleship", FrigateType |
DestroyerType |
CruiserType |
BattlecruiserType |
BattleshipType];
