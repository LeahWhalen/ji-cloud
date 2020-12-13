const {getMediaUrl_UI} = require("../../config/js/src/lib");

const isDev = process.env["NODE_ENV"] === "development";

export const MEDIA_UI = getMediaUrl_UI(isDev);

export const mediaUi = (path:string):string => `${MEDIA_UI}/${path}`;