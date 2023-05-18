interface ShrekSuperSlamCharacterAttackProjectile {
    x_vector: number;
    angle: number;
    arc: number;
    homing1: number;
    homing2: number;
    homing3: number;
}

interface ShrekSuperSlamCharacterAttackHitbox {
    delay: number;
    width: number;
    radius: number;
    offset: number;
}

interface ShrekSuperSlamCharacterAttack {
    damage1: number;
    damage2: number;
    damage3: number;
    disabled: boolean;
    endlag: number;
    fall_speed: number;
    hitboxes: ShrekSuperSlamCharacterAttackHitbox[];
    hits_otg: boolean;
    intangible: boolean;
    knocks_down: boolean;
    readonly name: string;
    projectiles: ShrekSuperSlamCharacterAttackProjectile[];
}

export interface NamedShrekSuperSlamCharacterAttack {
    readonly [name: string]: ShrekSuperSlamCharacterAttack;
}

export interface ShrekSuperSlamCharacterAttackCollection {
    readonly [character: string]: NamedShrekSuperSlamCharacterAttack;
}

interface ShrekSuperSlamStageProperty {}

export interface NamedShrekSuperSlamStageProperty {
    readonly [name: string]: ShrekSuperSlamStageProperty;
}

export interface ShrekSuperSlamStageCollection {
    readonly [stage: string]: NamedShrekSuperSlamStageProperty;
}