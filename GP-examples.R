library(ggplot2)  # what's this mean?
testFrame = expand.grid(
  x1 = seq(-6,10,0.2),
  x2 = seq(-10,10,0.2)
  )

testFrame$y = with(testFrame,
                   10*exp(-((x1-3)^2+(x2+5)^2)/10)+
                     15*exp(-((x1+3)^2+(x2-4)^2)/2)+
                     -20*exp(-((x1)^2+(x2)^2)/0.5)+
                     -12*exp(-((x1-7)^2+(x2-6)^2)/3))

ggplot(data=testFrame,aes(x=x1,y=x2,z=y,))+
  geom_tile(aes(fill = y)) + #stat_contour(aes(color=y))+
  scale_fill_gradient2(low = "red", high = "blue",mid="white")+
  #scale_color_gradient(low = "red", high = "blue")+
  #geom_point(size=2,alpha=0.75)+
  theme_bw()
