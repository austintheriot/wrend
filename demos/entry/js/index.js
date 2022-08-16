import '../css/style.scss';

import("../pkg/index.js").catch(console.error).then((module) => module.entry());
