#!/bin/env/awk -f
BEGIN{FS=":";t=0;}
{a=substr($1,5,6);b=1;
split($2,r,";");
for(i=1;i<=length(r);i++){
split(r[i],c,",")
for(j=1;j<=length(c);j++){
split(c[j],d," ");
if(d[2]=="blue"){if(d[1]>14){b=0;break}}
else if(d[2]=="red"){if(d[1]>12){b=0;break}}
else if(d[2]=="green"){if(d[1]>13){b=0;break}}}}if(b)t+=a;}
END{print t}
