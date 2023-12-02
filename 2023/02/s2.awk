#!/bin/env/awk -f
BEGIN{FS=":";t=0}{
r=0;b=0;g=0;p=0;
split($2,o,";");
for(i=1;i<=length(o);i++){
split(o[i],c,",")
for(j=1;j<=length(c);j++){
split(c[j],d," ");
if(d[2]=="blue"){
if(d[1]>b){b=d[1];}} 
else if(d[2]=="red"){if(d[1]>r){r=d[1];}}
else if(d[2]=="green"){if(d[1]>g){g=d[1];}}}}
p=g*b*r;t+=p}
END{print t}
