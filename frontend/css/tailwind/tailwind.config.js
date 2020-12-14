const {getMediaUrl_UI} = require("../../../config/js");
const isDev = process.env["NODE_ENV"] === "development";
const MEDIA_UI = getMediaUrl_UI(isDev);
console.log(`compiling tailwind...`);

/*
    the ji- prefixes are used for rem
    and intended for inside our scaled module areas
    the forumula is that it's pixels / 10
    so 123px becomes 12.3rem

    parseTheme will automatically add these for each px value
*/

function parseTheme(theme) {
    const PREFIX = "ji-";


    function parse(obj) {
        const keys = Object.keys(obj);

        keys.forEach(key => {
            const value = obj[key];
            if(typeof value === "object") {
                parse(value);
            } else if(typeof value === "string") {
                if(value.indexOf("px") !== -1) {
                    const newValue = value
                        .split(" ")
                        .map(word => {

                            const idx = word.lastIndexOf("px");
                            if(idx !== -1 && idx === word.length - 2) {
                                const nStr= word.substring(0, idx);
                                const nPx = parseFloat(nStr);
                                if(isNaN(nPx)) {
                                    console.error(`CAN'T CONVERT PX TO REM: key: [${key}] word: [${word}] full value: [${value}]`);
                                    process.exit(1);
                                } else {
                                    const nRem = nPx / 10;
                                    return `${nRem}rem`;
                                }
                            }
                            return word;
                        })
                        .join(" ");

                    const newKey = PREFIX + key;
                    obj[newKey] = newValue;
                }

            }
        });
    }

    parse(theme);

    return theme;
}

function getSpacing() {
  const ret = {};
  for(let i = 0; i < 100; i += .5) {
    ret[`${i}`] = `${(i / 4) * 16}px`;
  }

  return ret;
}
module.exports = {
  purge: [
    '../../templates/**/*.html',
  ],
  theme: parseTheme({
    extend: {
      colors: {
        jiblueLight: '#83aef7',
        jiblueMedium: '#6698ed',
        jiblueDark: '#2b54b8',
        jiblueadd: '#dae7fd',
        jinumberblue: '#c4dbfe',
        jigreen: '#61D592',
        jibuttonBlue:'#5590fc',
        jibackgroundBlue: "#e6f0ff",
        jibluelighter: "#a6c6f8",
        jiborderGrey: "#e5e7ef",
        jibackgroundGrey:'#f8f9fd',
        jigreyinputborder:'#d3d3d3;',
        jierrorred: '#e36486',
        jierrordarkred: '#ed464e',
        jiyellowbackground: '#fdf5d0',
        jiyellowstar:'#fccd63',
        jigreyicon: '#d6d8de',
        jiorange:'#fc7551',
        jiorangehover:"#ea685d",
        jititleorange: '#fd6220',
        jisideblue:'#def4ff',
        jilogingrey: '#f7f7f7',
        jisignupgrey: '#dee1eb',
        jidarkgrey: '#e2e5eb',
        jibuttongreen: '#72cb91',
        jigooglegrey: '#f0f1f4',
        jicreatorblue: '#edf7ff',
        jigrey: '#a1a8ad',
        jigreyfocus: '#d8d8d8',
        jiheadergrey: '#4a4a4a',
        jioverlayblue: '#e5f3ff',
        jiggreen: '#42cc7a',
        jiimagegrey: '#f8f8f8',
        jicolorborder: '#c7d3db',
        jicolorpickeryellow:'#fff445',
        jicolorpickerorange: '#fac72d',
        jicolorpickerorangedark: '#feae2a',
        jicolorpickerred: '#f34826',
        jicolorpickerpink: '#fb178d',
        jicolorpickerpinkdark: '#da0f63',
        jicolorpickerfuschia: '#f74ac8',
        jicolorpickerpurple: '#9517ac',
        jicolorpickerpurpledark: '#7a28fb',
        jicolorpickerbluedark: '#414cb3',
        jicolorpickerblue: '#2d9bf0',
        jicolorpickerteal: '#22cdd4',
        jicolorpickergreendark: '#18a789',
        jicolorpickergreen: '#8fd150',
        jicolorpickerlime: '#cfe741',
        jicolorpickerbluelight:'#bbccf8',
        jicolorpickergreyblue:'#dce9f5',
        jicolorpickergreylight:'#e6e6e6',
        jicolorpickergrey:'#808080',
        jimemoryblue:'#d1e8ff',
        jimemorybluehover:'#387af4',
        jimemorybackground: '#f8fbfd',
        jimemoryborder: '#c1d8f8',
        jimemorycardborder: '#2565d5',
        jimemorypink: '#ffdede',
        jimemorypinkbutton: '#ea5a80',
        jimemorypinkborder: '#e36486',
        jimemorygreenlight: '#eafaf0',
        jimemorycardgreen: '#7fd29c',
        jibackgroundyellow: '#fff8d1',
        jimemoryborderblue: '#6ea4fc',
        jigreentooltipbackground: '#f0fcf5',
        jibluehover:'#d6e6fd',

      },

      fontSize: {
        14:'14px',
        18: '18px',
        xs: '12px',
        sm: '14px',
        base: '16px',
        lg: '18px',
        xl: '20px',
        '2xl': '24px',
        '3xl': '30px',
        '4xl': '36px',
        '5xl': '48px',
        '6xl': '64px',
      },
      fontFamily: {
        poppins: 'Poppins',
        sans: 'Roboto',
        heebo: 'Heebo',

      },
      width: {
        1: '1px',
        2: '2px',
        10: '10px',
        30: '30px',
        41: '41px',
        44: '44px',
        48: '48px',
        56: '56px',
        68: '68px',
        72: '72px',
        76: '76px',
        83.5: '83.5px',
        84: '84px',
        89: '89px',
        96: '96px',
        102: '102px',
        110: '110px',
        112: '112px',
        117: '117px',
        124: '124px',
        128: '128px',
        132: '132px',
        136: '136px',
        139: '139px',
        146: '146px',
        150: '150px',
        152: '152px',
        160: '160px',
        176: '176px',
        186: '186px',
        190: '190px',
        200: '200px',
        212: '212px',
        217: '217px',
        236: '236px',
        240: '240px',
        254: '254px',
        256:'256px',
        259: '259px',
        270: '270px',
        272: '272px',
        274: '274px',
        288: '288px',
        296: '296px',
        297: '297px',
        306: '306px',
        312: '312px',
        320: '320px',
        325: '325px',
        344: '344px',
        360: '360px',
        362: '362px',
        389: '389px',
        393: '393px',
        408: '408px',
        419: '419px',
        480: '480px',
        504: '504px',
        516: '516px',
        624: '624px',
        640: '640px',
        763: '763px',
        867: '867px',
        992: '992px',
        1212: '1212px',
        1666:'1666px',
        1688: '1688px',
        '50p': '50%',
        '1/7': '14.28%'


      },
      maxWidth: {
        190: '190px',
        236: '236px',
        480: '480px',

      },
      minWidth: {
        112: '112px',
        297: '297px',
        300: '300px',
      },
      height: {
        1: '1px',
        10: '10px',
        20: '20px',
        24: '24px',
        32: '32px',
        48: '48px',
        56: '56px',
        64: '64px',
        68: '68px',
        83.5: '83.5px',
        89: '89px',
        96: '96px',
        102: '102px',
        110: '110px',
        114: '114px',
        124: '124px',
        128: '128px',
        136: '136px',
        140: '140px',
        146: '146px',
        160: '160px',
        167:'167px',
        177: '177px',
        180: '180px',
        185: '185px',
        190: '190px',
        192: '192px',
        208: '208px',
        216: '216px',
        220: '220px',
        262: '262px',
        293: '293px',
        312: '312px',
        344: '344px',
        362: '362px',
        365: '365px',
        383: '383px',
        387: '387px',
        521: '521px',
        537: '537px',
        581: '581px',
        612: '612px',
        650: '650px',
        668: '668px',
        681: '681px',
        696: '696px',
        796: '796px',
        896: '896px',
        935: '935px',

      },
      rotate:{
        10: '10deg',
      },

      backgroundPosition:{
        'right-center':"265px center"
      },
      maxHeight: {
        284: '284px',
        681: '681px',
        83.5: '83.5px',
      },
      minWidth: {
        516: '516px'
      },
      maxWidth: {
        83.5: '83.5px',
        1212: '1212px'
      },
      borderRadius: {
        8: '8px',
        10: '10px',
        12: '12px',
        14: '14px',
        16: '16px',
        20: '20px',
        24: '24px',
        32: '32px',
        36: '36px',
      },
      inset: {
        10: '10px',
        15: '15px',
        20:'20px',
        25: '25px',
        26: '26px',
        40:'40px',
        45: '45px',
        10:'10px',
        16:'16px',
        20: '20px',
        '15p':'15%',
        25:'25%',
        '-10': '-10px',
        50:'50px',
        '50p':'50%',
        '70p': '70%',
        70: '70px',
        95: '95px',
        115: '115px',
        '-120': '-120px',
        160: '160px',
        '-185':'-185px',
        240: '240px',
        '-296': '-296px',
        500: '500px',

      },
      borderWidth: {
        3: '3px'
      },
      padding:{
        25: '25px',
        60:'60px',
        80: '80px',
        180: '180px',
        255: '255px',
        '16:9': '56.25%'
      },
      margin:{
        '-10': '-10px',
        '-15': '-15px',
        '25p': '25%',
        '33p': '33%',
        50: '50px',
        100: '100px',
        138: '138px',

      },
      cursor:{
        grab: 'grab'
      },
      spacing: getSpacing(),
      boxShadow:{
        'memorycard': '0 3px 3px 0 rgba(0, 0, 0, 0.06)',
        'memorypreview': '0 3px 6px 0 rgba(0, 0, 0, 0.16)'
      },
      zIndex: {
        '2000' : 2000
      },
      gap: {
      '24px': '24px',
      '48px': '48px',
     },
     minHeight:{
       32: '32px'
     },

      backgroundImage: theme => ({
        'shapes': "url('https://i.ibb.co/g9N7MLy/shapes-1.png')",
        'poster': `url('${MEDIA_UI}/Icn_Module_Poster.svg')`

        })
    },
  }),
  variants: {
    backgroundColor: ['responsive', 'hover', 'focus', 'active', 'group-hover', 'disabled'],
    border: ['responsive', 'hover', 'focus', 'active', 'group-hover'],
    transitionProperty: ['responsive', 'motion-safe', 'motion-reduce'],
    borderWidth: ['hover', 'focus'],
    borderStyle: ['hover'],
  },
  plugins: [],
}
