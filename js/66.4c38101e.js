"use strict";(self["webpackChunkShab_aint"]=self["webpackChunkShab_aint"]||[]).push([[66],{8066:function(n,e,r){r.r(e),r.d(e,{__wbg_buffer_0be9fb426f2dd82b:function(){return t.K7},__wbg_error_4bb6c2a97407129a:function(){return t.kF},__wbg_new_4e8d18dbf9cd5240:function(){return t.W$},__wbg_new_59cb74e423758ede:function(){return t.h9},__wbg_new_7c995f2adeba6fb5:function(){return t.yw},__wbg_newwithbyteoffsetandlength_85b7ce82b001ea08:function(){return t.u4},__wbg_push_3f7c76b58919ce0d:function(){return t.Hs},__wbg_stack_558ba5917b466edd:function(){return t.Dz},__wbindgen_json_parse:function(){return t.t$},__wbindgen_json_serialize:function(){return t.r1},__wbindgen_memory:function(){return t.oH},__wbindgen_object_drop_ref:function(){return t.ug},__wbindgen_throw:function(){return t.Or},extract_character_attacks:function(){return t.sz},recreate_game_files:function(){return t.At}});var t=r(4785)},4785:function(n,e,r){r.d(e,{At:function(){return A},Dz:function(){return O},Hs:function(){return $},K7:function(){return j},Or:function(){return U},W$:function(){return I},h9:function(){return E},kF:function(){return H},oH:function(){return q},r1:function(){return z},sz:function(){return x},t$:function(){return T},u4:function(){return C},ug:function(){return D},yw:function(){return S}});r(3767),r(8585),r(8696),r(7658);var t=r(3668);n=r.hmd(n);const u="undefined"===typeof TextDecoder?(0,n.require)("util").TextDecoder:TextDecoder;let o=new u("utf-8",{ignoreBOM:!0,fatal:!0});o.decode();let c=null;function i(){return null!==c&&c.buffer===t.memory.buffer||(c=new Uint8Array(t.memory.buffer)),c}function f(n,e){return o.decode(i().subarray(n,n+e))}const _=new Array(32).fill(void 0);_.push(void 0,null,!0,!1);let a=_.length;function l(n){a===_.length&&_.push(_.length+1);const e=a;return a=_[e],_[e]=n,e}function b(n){return _[n]}let d=0;const s="undefined"===typeof TextEncoder?(0,n.require)("util").TextEncoder:TextEncoder;let w=new s("utf-8");const g="function"===typeof w.encodeInto?function(n,e){return w.encodeInto(n,e)}:function(n,e){const r=w.encode(n);return e.set(r),{read:n.length,written:r.length}};function h(n,e,r){if(void 0===r){const r=w.encode(n),t=e(r.length);return i().subarray(t,t+r.length).set(r),d=r.length,t}let t=n.length,u=e(t);const o=i();let c=0;for(;c<t;c++){const e=n.charCodeAt(c);if(e>127)break;o[u+c]=e}if(c!==t){0!==c&&(n=n.slice(c)),u=r(u,t,t=c+3*n.length);const e=i().subarray(u+c,u+t),o=g(n,e);c+=o.written}return d=c,u}let y=null;function m(){return null!==y&&y.buffer===t.memory.buffer||(y=new Int32Array(t.memory.buffer)),y}function v(n){n<36||(_[n]=a,a=n)}function p(n){const e=b(n);return v(n),e}function k(n,e){const r=e(1*n.length);return i().set(n,r/1),d=n.length,r}function x(n,e,r){var u=k(n,t.__wbindgen_malloc),o=d,c=k(e,t.__wbindgen_malloc),i=d,f=t.extract_character_attacks(u,o,c,i,r);return p(f)}function A(n,e,r,u){var o=k(n,t.__wbindgen_malloc),c=d,i=k(e,t.__wbindgen_malloc),f=d,_=t.recreate_game_files(o,c,i,f,r,l(u));return p(_)}const T=function(n,e){var r=JSON.parse(f(n,e));return l(r)},z=function(n,e){const r=b(e);var u=JSON.stringify(void 0===r?null:r),o=h(u,t.__wbindgen_malloc,t.__wbindgen_realloc),c=d;m()[n/4+1]=c,m()[n/4+0]=o},D=function(n){p(n)},E=function(){var n=new Error;return l(n)},O=function(n,e){var r=b(e).stack,u=h(r,t.__wbindgen_malloc,t.__wbindgen_realloc),o=d;m()[n/4+1]=o,m()[n/4+0]=u},H=function(n,e){try{console.error(f(n,e))}finally{t.__wbindgen_free(n,e)}},S=function(){var n=new Array;return l(n)},$=function(n,e){var r=b(n).push(b(e));return r},j=function(n){var e=b(n).buffer;return l(e)},C=function(n,e,r){var t=new Uint8Array(b(n),e>>>0,r>>>0);return l(t)},I=function(n){var e=new Uint8Array(b(n));return l(e)},U=function(n,e){throw new Error(f(n,e))},q=function(){var n=t.memory;return l(n)}},3668:function(n,e,r){var t=r.w[n.id];n.exports=t;r(4785);t[""]()}}]);