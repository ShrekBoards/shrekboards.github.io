<template>
  <div class="fields">
    <Navbar />
    <div class="row">
      <div class="col s6 offset-s3">
        <h2>Fields</h2>
        <p>This section details what the different fields within the app do within the game.</p>
        <p>Some of this is speculation and guesswork! Please report if things do not work as written.</p>

        <h4>Attack Fields</h4>

        <h5>Numeric fields</h5>
        <table class="striped">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>aim_assist_strength</td>
              <td>Aim tracking, lower numbers (to a minimum of -1) increase the attack's tracking.</td>
            </tr>
            <tr>
              <td>aim_assist_strength2</td>
              <td>Appears to be similar to aim_assist_strength?</td>
            </tr>
            <tr>
              <td>aim_range</td>
              <td>The distance at which the attack homes in on the opponent.</td>
            </tr>
            <tr>
              <td>allow_movement</td>
              <td>
                If non-zero, allows characters to move during the animation.<br/>
                Values appear to be between 0 and 3, inclusive.
              </td>
            </tr>
            <tr>
              <td>animation_speed</td>
              <td>The speed of the attack animation (in seconds) for the attack.</td>
            </tr>
            <tr>
              <td>animation_speed_on_hit</td>
              <td>The speed of the attack animation (in seconds) if the attack hits.</td>
            </tr>
            <tr>
              <td>animation_speed_on_miss</td>
              <td>The speed of the attack animation (in seconds) if the attack misses.</td>
            </tr>
            <tr>
              <td>charge</td>
              <td>The time (in seconds) for a charge attack to fully charge.</td>
            </tr>
            <tr>
              <td>charge_effect</td>
              <td>The time (in seconds) for the charging animation - the orange flash - to complete.</td>
            </tr>
            <tr>
              <td>damage1</td>
              <td>
                The damage dealt by the attack.<br/>
                The user's SLAM bar fills after dealing 100 damage.<br/>
                Negative numbers reduce the user's SLAM bar.
              </td>
            </tr>
            <tr>
              <td>damage2</td>
              <td>Unknown, seems to be used instead of damage1 rarely. Set this to the same value as damage1.</td>
            </tr>
            <tr>
              <td>damage3</td>
              <td>Unknown, seems to be used instead of damage1 rarely. Set this to the same value as damage1.</td>
            </tr>
            <tr>
              <td>duration</td>
              <td>
                The higher the number, the longer the move will continue for.<br/>
                Hitboxes don't always follow along.
              </td>
            </tr>
            <tr>
              <td>endlag</td>
              <td>The amount of time (in seconds) the character is inactive for after the move completes.</td>
            </tr>
            <tr>
              <td>fall_speed</td>
              <td>
                Controls vertical movement in the attack.<br/>
                Positive numbers make the character go up, the larger the faster.<br/>
                Negative goes down, the smaller the faster.
              </td>
            </tr>
            <tr>
              <td>horizontal_knockback1</td>
              <td>
                How far the attack landing pushes the opponent away.<br/>
                Positive numbers push an opponent away, negative numbers pull toward.
              </td>
            </tr>
            <tr>
              <td>horizontal_knockback2</td>
              <td>
                Unknown, seems to be used instead of horizontal_knockback1 rarely.
              </td>
            </tr>
            <tr>
              <td>horizontal_knockback3</td>
              <td>
                Unknown, seems to be used instead of horizontal_knockback1 rarely.
              </td>
            </tr>
            <tr>
              <td>invincibility</td>
              <td>
                The amount of time, in seconds, that the character is invincible after performing the attack.<br/>
                The invincibility is cancelled early if the player performs another attack.<br/>
                This is often used in conjunction with the endlag field on SLAM attacks.
              </td>
            </tr>
            <tr>
              <td>max_move_curving</td>
              <td>
                Tracing / curving of attacks, larger numbers will follow opponents and do more elaborate curves during the attack.<br/>
                Minimum value of 1, maximum 100.<br/>
                Non-curving moves have this set to 99999, which turns it off.
              </td>
            </tr>
            <tr>
              <td>move_angle</td>
              <td>
                The angle of the move.<br/>
                Moving in a straight line is 1, decimal numbers between 0 and 1 go diagonally.
              </td>
            </tr>
            <tr>
              <td>multi_hit_speed</td>
              <td>
                How many times a second a multi-hit move hits.<br/>
                If set to zero, the move will be a single-hit move.
              </td>
            </tr>
            <tr>
              <td>shield_break_stun_time</td>
              <td>The amount of time (in seconds) an opponent is stunned for if the move breaks their shield.</td>
            </tr>
            <tr>
              <td>stun</td>
              <td>
                The amount of time (in seconds) that the attacks stuns the opponent for.<br/>
                Stun in this context refers to the state where a character is unable to act, and has stars circling their heads.
              </td>
            </tr>
            <tr>
              <td>vertical_follow_opponent</td>
              <td>
                Follows the opponent vertically if enabled. Examples are Gingerbread Man, Humpty Dumpty and G-nome's charged specials.<br/>
                Move must also lock-on.
              </td>
            </tr>
            <tr>
              <td>vertical_knockback1</td>
              <td>
                How far the attack landing pushes the opponent upwards.<br/>
                Positive numbers push an opponent upwards, negative numbers pull down to the ground.
              </td>
            </tr>
            <tr>
              <td>vertical_knockback2</td>
              <td>
                Unknown, sometimes used instead of vertical_knockback1. Does not seem to affect high wall attacks.
              </td>
            </tr>
            <tr>
              <td>vertical_knockback3</td>
              <td>
                Unknown, sometimes used instead of vertical_knockback1.
              </td>
            </tr>
          </tbody>
        </table>

        <h5>Tickbox fields</h5>
        <table class="striped">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>allow_charge</td>
              <td>
                Allows charging of some uncharged strong attacks and slams, and ground throws.
              </td>
            </tr>
            <tr>
              <td>disable_curving</td>
              <td>
                If enabled, prevents moves from curving during the attack, forcing them into a straight line.
              </td>
            </tr>
            <tr>
              <td>disabled</td>
              <td>If checked, inputting the move causes nothing to happen.</td>
            </tr>
            <tr>
              <td>hits_otg</td>
              <td>If checked, the move hits characters that are in the knocked-down blue state.</td>
            </tr>
            <tr>
              <td>horizontal_lock_on</td>
              <td>
                Enables horizontal lock-on, for example Pinnochio's uncharged strong attack.
              </td>
            </tr>
            <tr>
              <td>ignore_gravity</td>
              <td>If checked, the move is unaffected by gravity (e.g. Captain Hook's airdash).</td>
            </tr>
            <tr>
              <td>intangible</td>
              <td>If checked, the move doesn't seem to do anything? This one needs more research.</td>
            </tr>
            <tr>
              <td>is_slam</td>
              <td>If checked, the move will SLAM the opponent if it connects and the user's SLAM bar is full.</td>
            </tr>
            <tr>
              <td>is_slam_at_any_percent</td>
              <td>If checked, the move will SLAM the opponent if it connects, no matter the user's SLAM bar percentage. It then empties the user's SLAM bar.</td>
            </tr>
            <tr>
              <td>knocks_down</td>
              <td>If checked, the move connecting puts the opponent in the blue knocked-down state.</td>
            </tr>
            <tr>
              <td>lock_position</td>
              <td>If checked, the direction the user is facing will be locked in place during the move.</td>
            </tr>
            <tr>
              <td>maintain_direction</td>
              <td>If checked, the user of the move will retain the direction they were facing. Otherwise, the user will turn to face the opponent once the move completes.</td>
            </tr>
            <tr>
              <td>no_opponent_contact</td>
              <td>If checked, the move will not make contact with opponents. It will still make contact with walls and throwables.</td>
            </tr>
            <tr>
              <td>no_opponent_contact2</td>
              <td>
                If enabled, makes it so you maintain no contact after hitting the move.
              </td>
            </tr>
            <tr>
              <td>shield_breaks</td>
              <td>If checked, the move will break shields on contact.</td>
            </tr>
            <tr>
              <td>shield_breaks_without_stun</td>
              <td>If checked, the move will break shields on contact, but the opponent will not be stunned.</td>
            </tr>
          </tbody>
        </table>

        <h4>Hitbox Fields</h4>
        <table class="striped">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>delay</td>
              <td>The amount of time (in seconds) it takes for the hitbox to become active from when the attack starts.</td>
            </tr>
            <tr>
              <td>arc</td>
              <td>
                The angle the circle 'wraps around' the character making the attack.<br/>
                0 is a semicircle in front of the character, 1 is a thin line directly in front, and -1 is a full circle around.
              </td>
            </tr>
            <tr>
              <td>radius</td>
              <td>
                How far the hitbox stretches out from the character.<br/>
                This number cannot be less than zero, or the game crashes.
              </td>
            </tr>
            <tr>
              <td>horizontal_angle</td>
              <td>
                The horizontal angle of the hitbox.<br/>
                Negative numbers go right, positive left, zero is directly in front of the player.
              </td>
            </tr>
          </tbody>
        </table>

        <h4>Projectile Fields</h4>
        <p>
          The projectile fields are generally not very well understood and these could be incorrect. Things such as the damage
          dealt by the projectile are controlled by its encompassing attack.
        </p>
        <table class="striped">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>angle</td>
              <td>Along with arc, has some bearing on the angle the projectile is fired at.</td>
            </tr>
            <tr>
              <td>arc</td>
              <td>Along with angle, has some bearing on the angle the projectile is fired at.</td>
            </tr>
            <tr>
              <td>homing1</td>
              <td>Controls the degree in which the projectile homes in on the opponent.</td>
            </tr>
            <tr>
              <td>homing2</td>
              <td>Controls the degree in which the projectile homes in on the opponent.</td>
            </tr>
            <tr>
              <td>homing3</td>
              <td>Controls the degree in which the projectile homes in on the opponent.</td>
            </tr>
            <tr>
              <td>x_vector</td>
              <td>Controls the projectile's speed in the x-axis</td>
            </tr>
          </tbody>
        </table>

        <h4>Player Physics</h4>
        <p>
          Most of the player physics values come from the game itself, from a
          debug method that was left in. We therefore don't know exactly what
          each does, but the names are "official Shaba" and somewhat descriptive.
        </p>
        <p>
          The following fields are the exception, and are found from reverse-engineering
          like the other fields:
        </p>
        <table class="striped">
          <thead>
            <tr>
              <th>Name</th>
              <th>Description</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>knock_down_time</td>
              <td>The amount of time, in seconds, the character stays knocked-down (without mashing).</td>
            </tr>
            <tr>
              <td>mash_value</td>
              <td>
                An additional value to decrement the knocked-down time when the player mashes.<br/>
                Due to the way the code works, setting this value to 1 makes mashing do nothing.<br/>
                Setting this value to less than 1 makes mashing slower than not.<br/>
                Setting this value to negative would make mashing keep the player knocked-down longer than not mashing.<br/>
              </td>
            </tr>
          </tbody>
        </table>

        <h4>Unknown Fields</h4>
        <p>
          The attack objects are a collection of about 300 bytes, comprising of
          many fields, mostly in the form of decimal numbers. Which numbers
          correspond to what property of the attack is typically worked out
          through trial-and-error, by tweaking the values and seeing what
          happens.
        </p>
        <p>
          As a result, many of these fields have had their location identified,
          but not their purpose. If you would like to access these unknown fields
          to play around with them, tick <b>Show unknown fields</b> in the upload.
          You can then modify these values, and work out what they do for yourself.
          If you manage to deduce what a field does, please open an
          <a href="https://github.com/ShrekBoards/shrek-superslam/issues">issue</a>
          on the main library explaining what the field does, or message kirby.
        </p>
        <p>
          Hopefully this will allow new fields to be discovered much sooner!
        </p>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import Navbar from '@/components/Navbar.vue';

export default defineComponent({
  name: 'fields-help-page',
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