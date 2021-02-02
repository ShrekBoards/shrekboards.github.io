import('../pkg/index').then(s3web => {
    console.log("All modules loaded");
    document.getElementById("theform")!.onsubmit = function(event) {
        // Get crap out of form
        let f1 = (<HTMLInputElement>document.getElementById("masterdir"));
        let f2 = (<HTMLInputElement>document.getElementById("masterdat"));
        if (f1 != null && f2 != null) {
            var p1 = f1.files![0].arrayBuffer().then(buffer => new Uint8Array(buffer));
            var p2 = f2.files![0].arrayBuffer().then(buffer => new Uint8Array(buffer));
            var x = {};
            Promise.all([p1, p2]).then((values) => {
                var gameconsole = 0;
                document.getElementsByName("gameconsole").forEach((item, _) => {
                    if ((<HTMLInputElement>item).checked) {
                        gameconsole = parseInt((<HTMLInputElement>item).value);
                    }
                })
        
                // Submit to wasm function
                x = s3web.entry1(values[1], values[0], gameconsole);
            });
        }

    
        // Disable the event resubmission
        if (event.preventDefault)
            event.preventDefault();
        else
            return false;
    }
});