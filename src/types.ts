interface ShrekSuperSlamCharacterAttack {}

interface ShrekSuperSlamCharacterPhysics {}

interface ShrekSuperSlamCharacterAttacksCollection {
    readonly [name: string]: ShrekSuperSlamCharacterAttack;
}

interface ShrekSuperSlamCharacter {
    readonly [field: string]: ShrekSuperSlamCharacterAttacksCollection | ShrekSuperSlamCharacterPhysics;
}

export interface ShrekSuperSlamCharacterCollection {
    readonly [character: string]: ShrekSuperSlamCharacter;
}

interface ShrekSuperSlamStageSpitter {}

interface ShrekSuperSlamStageSpitterCollection {
    readonly [name: string]: ShrekSuperSlamStageSpitter;
}

interface ShrekSuperSlamStage {
    readonly [field: string]: ShrekSuperSlamStageSpitterCollection;
}

export interface ShrekSuperSlamStageCollection {
    readonly [stage: string]: ShrekSuperSlamStage;
}