<template>
  <div class="upload">
    <form id="theform">
      <p>MASTER.DAT: <input class="file-selector" type="file" id="masterdat" name="files[]" /></p>
      <p>MASTER.DIR: <input class="file-selector" type="file" id="masterdir" name="files[]" /></p>
      <input type="radio" id="gamecube" name="gameconsole" value=0>
      <label for="console">Gamecube</label><br>
      <input type="radio" id="pc" name="gameconsole" value=1>
      <label for="console">PC</label><br>
      <input type="radio" id="ps2" name="gameconsole" value=2>
      <label for="console">PS2</label><br>
      <input type="radio" id="xbox" name="gameconsole" value=3>
      <label for="console">Xbox</label><br>
      <input type="submit">
    </form>
  </div>
</template>