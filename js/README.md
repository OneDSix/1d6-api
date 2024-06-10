# JavaScript/Typescript SDK

Licensed under `AGPL-3.0`.
All contributions welcome.

## Compiling

First make sure you have at least npm `>=10.7.0`, node `>=18.17.1`, and git `>=2.37.3`,\
Then run;

```shell
git clone https://github.com/OneDSix/1d6-api.git
cd ./1d6-api/js
npm i
npm run build
```

Lastly, grab the distributable from `./dist/bundle.*.js`; either `bundle.node.js` or `bundle.web.js`.

## Examples

TODO

## Contributing

I recommend using [LiveServer](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) and opening [`test.html`](/js/test.html) in your browser, as this contains the SDK and will attempt to log in with a given username/password combo.\
You can also use `test.html` as a starting point for anything you make, just link back here in the code.
