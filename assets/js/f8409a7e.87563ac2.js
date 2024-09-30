"use strict";(self.webpackChunkwebsite=self.webpackChunkwebsite||[]).push([[6903],{8606:(e,s,n)=>{n.r(s),n.d(s,{assets:()=>d,contentTitle:()=>l,default:()=>h,frontMatter:()=>a,metadata:()=>t,toc:()=>c});var i=n(4848),o=n(8453);const a={},l="How To",t={id:"intro",title:"How To",description:"Add a color",source:"@site/docs/intro.mdx",sourceDirName:".",slug:"/intro",permalink:"/colornames-i18n/docs/intro",draft:!1,unlisted:!1,editUrl:"https://github.com/dev-easily/colornames-i18n/blob/main/docs/intro.mdx",tags:[],version:"current",frontMatter:{},sidebar:"docs"},d={},c=[{value:"Add a color",id:"add-a-color",level:2},{value:"Add a locale",id:"add-a-locale",level:2},{value:"Add/Change a translation",id:"addchange-a-translation",level:2},{value:"Use VSCode to avoid encoding problem",id:"use-vscode-to-avoid-encoding-problem",level:3},{value:"Add an alternative",id:"add-an-alternative",level:3},{value:"Use a color",id:"use-a-color",level:2},{value:"TODO",id:"todo",level:2}];function r(e){const s={a:"a",admonition:"admonition",code:"code",h1:"h1",h2:"h2",h3:"h3",header:"header",img:"img",input:"input",li:"li",ol:"ol",p:"p",strong:"strong",...(0,o.R)(),...e.components};return(0,i.jsxs)(i.Fragment,{children:[(0,i.jsx)(s.header,{children:(0,i.jsx)(s.h1,{id:"how-to",children:"How To"})}),"\n",(0,i.jsx)(s.h2,{id:"add-a-color",children:"Add a color"}),"\n",(0,i.jsxs)(s.p,{children:["You should add colors in ",(0,i.jsx)(s.a,{href:"https://github.com/meodai/color-names",children:"https://github.com/meodai/color-names"}),".\nThis tool(Color Names i18n) will automatically update them."]}),"\n",(0,i.jsx)(s.admonition,{type:"info",children:(0,i.jsxs)(s.p,{children:["Only the ",(0,i.jsx)(s.code,{children:"bestof"})," colors will be updated. So if you want to add a color, make sure it's the best!"]})}),"\n",(0,i.jsx)(s.h2,{id:"add-a-locale",children:"Add a locale"}),"\n",(0,i.jsxs)(s.p,{children:["If you click on the first column of ",(0,i.jsx)(s.a,{href:"http://localhost:3000/colornames-i18n/",children:"the table"}),", Color Names i18n will take you to ",(0,i.jsx)(s.code,{children:"deepl.com"})," and translate the name from english to\n",(0,i.jsx)(s.code,{children:"the locale you selected"}),"(not supported yet)."]}),"\n",(0,i.jsx)(s.p,{children:(0,i.jsx)(s.img,{alt:"alt text",src:n(4129).A+"",width:"1493",height:"461"})}),"\n",(0,i.jsx)(s.p,{children:"Select a target language on deepl.com, and the location of the browser will change."}),"\n",(0,i.jsxs)(s.p,{children:["The url would be like ",(0,i.jsx)(s.code,{children:"https://www.deepl.com/en/translator#en/"}),(0,i.jsx)(s.strong,{children:"zh-hans"}),(0,i.jsx)(s.code,{children:"/SomeThingNeedsToBeTranslated"}),"."]}),"\n",(0,i.jsxs)(s.p,{children:["You should use the bold part(for example, ",(0,i.jsx)(s.strong,{children:"zh-hans"}),") as the locale, so when you click on the English color name, we will take you to the right deepl.com page."]}),"\n",(0,i.jsx)(s.p,{children:"But it's not a requirement, you can always add a locale that does not exist on deepl.com."}),"\n",(0,i.jsxs)(s.p,{children:["After you decide your mind, search ",(0,i.jsx)(s.code,{children:"colornames.i18n.csv"})," in this project, and add a column using VSCode."]}),"\n",(0,i.jsx)(s.admonition,{type:"warning",children:(0,i.jsxs)(s.p,{children:["Do not use Microsoft Excel. ",(0,i.jsx)(s.a,{href:"#use-vscode-to-avoid-encoding-problem",children:"See why ->"})]})}),"\n",(0,i.jsx)(s.h2,{id:"addchange-a-translation",children:"Add/Change a translation"}),"\n",(0,i.jsx)(s.h3,{id:"use-vscode-to-avoid-encoding-problem",children:"Use VSCode to avoid encoding problem"}),"\n",(0,i.jsxs)(s.p,{children:["Microsoft Excel will not save large amounts of characters when dealing with csv files, it will change them to ",(0,i.jsx)(s.code,{children:"_"}),", which is not acceptable."]}),"\n",(0,i.jsx)(s.p,{children:"Non-English language users have to take care of these kinds of problems every day. It's racist, for anyone who needs to type letters other than a-z or 0-9..."}),"\n",(0,i.jsxs)(s.p,{children:["So let's ",(0,i.jsx)(s.strong,{children:"abandon"})," it."]}),"\n",(0,i.jsxs)(s.p,{children:["VSCode with ",(0,i.jsx)(s.code,{children:"Edit csv"})," plugin is recommended, you can copy multiple lines and paste them like you did in Excel, what you see is what you save, without being fooled. Just like the way it should be."]}),"\n",(0,i.jsx)(s.p,{children:(0,i.jsx)(s.img,{alt:"search edit csv in vscode market",src:n(6467).A+"",width:"2166",height:"734"})}),"\n",(0,i.jsx)(s.p,{children:(0,i.jsx)(s.img,{alt:"do-not-use-excel-to-edit-csv",src:n(2807).A+"",width:"1196",height:"1037"})}),"\n",(0,i.jsx)(s.h3,{id:"add-an-alternative",children:"Add an alternative"}),"\n",(0,i.jsx)(s.p,{children:"Colors have different names in different countries. They are not standardized in all regions."}),"\n",(0,i.jsx)(s.p,{children:"Sometimes a color is more accurately or romantically named in your area. Or it may have several names, all of which fit perfectly."}),"\n",(0,i.jsx)(s.p,{children:"You can add a name like:"}),"\n",(0,i.jsx)(s.p,{children:(0,i.jsx)(s.code,{children:"Name1/Name2/Name3"})}),"\n",(0,i.jsxs)(s.p,{children:[(0,i.jsx)(s.code,{children:"Name1"})," will be used to display in columns, while other names will only be displayed when the user hovers over the cell."]}),"\n",(0,i.jsx)(s.p,{children:"In addition, if a color has more than one name, the cell displays a badge that serves as a reminder of the specificity of the color."}),"\n",(0,i.jsx)(s.p,{children:"Having multiple names for a color will always make it difficult to choose. So when enough people see the badge, someone will always come along and submit an ISSUE, and then everyone will discuss it and finally come to a consensus and fix it to one name."}),"\n",(0,i.jsx)(s.p,{children:"That's the intent of the badge."}),"\n",(0,i.jsx)(s.h2,{id:"use-a-color",children:"Use a color"}),"\n",(0,i.jsx)(s.p,{children:"Search and copy, even try to understand the meaning behind the colors, which will take you to a whole new world."}),"\n",(0,i.jsx)(s.h2,{id:"todo",children:"TODO"}),"\n",(0,i.jsxs)(s.ol,{className:"contains-task-list",children:["\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","\u9875\u9762\u70b9\u51fb\u53ef\u4ee5\u8df3\u8f6c\u5230 csv \u7684\u5bf9\u5e94\u884c\n",(0,i.jsx)(s.a,{href:"https://github.com/dev-easily/colornames-i18n/blob/de2b67ef21eaf417e317ad89900db99284cb9cf4/rust/i18n/colornames.i18n.csv#L8",children:"https://github.com/dev-easily/colornames-i18n/blob/de2b67ef21eaf417e317ad89900db99284cb9cf4/rust/i18n/colornames.i18n.csv#L8"})]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","\u2605 \u9875\u9762\u53ef\u4ee5\u663e\u793a\u591a\u79cd\u8bed\u8a00\uff0c\u4e0d\u4ec5\u662f\u4e2d\u6587"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","\u61d2\u52a0\u8f7d\uff0c\u5e76\u6dfb\u52a0\u8fc7\u6ee4\u548c\u641c\u7d22\u529f\u80fd"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","\u6dfb\u52a0\u4e00\u6279\u673a\u5668\u7ffb\u8bd1\uff0c\u5e76\u6dfb\u52a0\u673a\u5668\u7ffb\u8bd1\u7684\u6807\u8bb0"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",checked:!0,disabled:!0})," ","\u516c\u544a\u680f\u4fee\u6539\u4e3a\u6b63\u786e\u7684\u8fde\u63a5"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",checked:!0,disabled:!0})," ","docs \u589e\u52a0\u5982\u4f55\u6dfb\u52a0\u56fd\u9645\u5316\u7684\u6307\u5357"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","\u70b9\u51fb\u989c\u8272\u4ee3\u7801\u53ef\u4ee5\u590d\u5236\u5230\u526a\u5207\u677f"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","\u2605 \u589e\u52a0json\u8f93\u51fa\u652f\u6301"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","\u2605 \u589e\u52a0\u53ea\u8f93\u51fa\u67d0\u4e00\u79cd\u8bed\u8a00\u7684\u529f\u80fd"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","\u589e\u52a0\u4e00\u4e2a readme"]}),"\n",(0,i.jsxs)(s.li,{className:"task-list-item",children:[(0,i.jsx)(s.input,{type:"checkbox",disabled:!0})," ","github action \u81ea\u52a8\u66f4\u65b0 csv"]}),"\n"]})]})}function h(e={}){const{wrapper:s}={...(0,o.R)(),...e.components};return s?(0,i.jsx)(s,{...e,children:(0,i.jsx)(r,{...e})}):r(e)}},2807:(e,s,n)=>{n.d(s,{A:()=>i});const i=n.p+"assets/images/do-not-use-excel-to-edit-csv-fb05021345a8010c9d33fabaf37a59ba.png"},6467:(e,s,n)=>{n.d(s,{A:()=>i});const i=n.p+"assets/images/search-edit-csv-7fdcd92f348f862d3ea4c8528db0ea5d.png"},4129:(e,s,n)=>{n.d(s,{A:()=>i});const i=n.p+"assets/images/supported-codes-7e95340a902c9b03369b87fc0877ad4f.png"}}]);