set.seed(12345)
library(ggplot2)
library(MASS)
library(dplyr)
library(reshape2)
# library(fda)

SE <- function(Xi,Xj, l=1, s=1) (s^2)*exp(-(Xi - Xj)^2 / l ^ 2)
cov <- function(X, Y, l=1, s=1) outer(X, Y, SE, l, s)

# not-very efficient plotting function:

plotGP <- function(xRange,obs=c(),l,sigma,noise=0,samples=TRUE){
  x_predict <- seq(xRange[1],xRange[2],length.out=100)
  if(is.null(obs)){
    COV <- cov(x_predict,x_predict,l,sigma)
    values <- mvrnorm(3, rep(0, length=length(x_predict)), COV)
    dat <- data.frame(x=x_predict, t(values))
    dat <- melt(dat, id="x")
    
    ggplot(dat,aes(x=x,y=value)) +
      geom_rect(xmin=-Inf, xmax=Inf, ymin=-2*sigma, ymax=2*sigma, fill="grey80") +
      geom_line(aes(group=variable)) +   theme_bw() +
      ylab("output, f(x)") + xlab("input, x") +
      coord_cartesian(xlim=xRange,ylim =c(-5,5))
  } else {
    cov_xx_inv <- solve(cov(obs$x, obs$x,l,sigma) + noise^2 * diag(1, length(obs$x)))
    Ef <- cov(x_predict, obs$x,l,sigma) %*% cov_xx_inv %*% obs$y
    Cf <- cov(x_predict, x_predict,l,sigma) - cov(x_predict, obs$x,l,sigma)  %*% cov_xx_inv %*% cov(obs$x, x_predict,l,sigma)
    # predictions in data frame
    value <- mvrnorm(5, Ef, Cf)
    dat <- data.frame(x=x_predict, t(value))
    dat <- melt(dat, id="x")
    # Actual plotting bits:
    fakeData = data.frame(x=x_predict,y=Ef,lower=(Ef-2*sigma*sqrt(diag(Cf))),upper=(Ef+2*sigma*sqrt(diag(Cf))))
    ggplot(dat,aes(x=x,y=value)) +
      geom_ribbon(data=fakeData, aes(y=value,ymin=lower, ymax=upper), fill="grey80") +
      geom_line(aes(group=variable,color=variable)) + #REPLICATES
      geom_line(data=fakeData,aes(x=x,y=y), size=1) + #MEAN
      geom_point(data=obs,aes(x=x,y=y),size=2,shape=21) +  #OBSERVED DATA
      ylab("output, f(x)") + xlab("input, x") +
      coord_cartesian(xlim=xRange,ylim =c(-5,5)) +
      theme_bw()+theme(legend.position="none")
  }
}