<template>
  <div class="fields">
    <Navbar />
    <div class="row">
      <div class="col s6 offset-s3">
        <h2>Fields</h2>
        <p>This section details what the different fields within the app do within the game.</p>
        <p>Some of this is speculation and guesswork! Please report if things do not work as written.</p>
        <h5>Attack Names</h5>
        <ul class="list">
          <li>A <b>special</b> attack is the character's SLAM attack.</li>
          <li>The rest of the attacks should be self-explanatory from the name.</li>
        </ul>
        <h5>Attack Fields</h5>
        <ul class="list">
          <li>
            <b>damage1</b> is the damage dealt by the attack. The SLAM bar fills at 100 damage. Can be negative, which
            causes the SLAM bar to decrease when the move connects.
          </li>
          <li>
            The purposes of <b>damage2</b> and <b>damage3</b> are unknown. When trying to edit a move's damage, it is
            recommended to just edit all three fields to the same value. These two fields are used in some unknown
            circumstances, but the majority of calculations use damage1.
          </li>
          <li>
            The <b>endlag</b> is the number of seconds the character is inactive for after the move completes.
          </li>
          <li>
            The <b>fall_speed</b> controls vertical movement in the attack. Positive numbers make the character go
            up, the larger the fast, negative goes down, the smaller the faster.
          </li>
          <li>
            If a move is <b>disabled</b>, then inputting it causes nothing to happen.
          </li>
          <li>
            A move that <b>hits_otg</b> hits characters in the blue knocked-down state.
          </li>
          <li>
            If the <b>knocks_down</b> flag is set, the the move connecting puts the opponent in the blue knocked-down state.
          </li>
        </ul>
        <h5>Hitbox Fields</h5>
        <ul class="list">
          <li>
            The <b>delay</b> is the amount of time in seconds it takes for the hitbox to become active from when the attack
            starts.
          </li>
          <li>
            The <b>width</b> is the angle the circle 'wraps around' the character making the attack. Zero is a semicircle in
            front of the character, one is a thin line directly in front, and negative one is a full circle around.
          </li>
          <li>
            The <b>radius</b> is how far the hitbox stretches out from the character. This number cannot be less than zero, or
            the game crashes.
          </li>
        </ul>
        <h5>Projectile Fields</h5>
        <p>
          The projectile fields are generally not very well understood and these could be incorrect. Things such as the damage
          dealt by the projectile are controlled by its encompassing attack.
        </p>
        <ul class="list">
          <li>
            A projectile's <b>x_vector</b> controls its speed in the x-axis.
          </li>
          <li>
            The <b>angle</b> and <b>arc</b> parameters has some bearing on the angle the projectile is fired at.
          </li>
          <li>
            <b>homing1</b>, <b>homing2</b> and <b>homing3</b> all control the degree in which the projectile homes
            in on an opponent.
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'Fields',
  components: {
      Navbar,
  },
});
</script>

<style scoped>
ul.list {
  list-style: inside;
}

.list > li {
  list-style: inside;
  padding: 5px;
}
</style>