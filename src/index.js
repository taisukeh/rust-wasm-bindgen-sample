import("rustwasm_sample.js").then(module => {
    const { greet, hello, timed } = module;
    hello("hello!!");
    
    let v = timed( function (v, v2)  {
        console.log("a:" + v);
        console.log("b:" + v2);
    });
    console.log("" + v);
});
