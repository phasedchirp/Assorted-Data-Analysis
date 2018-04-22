library(rstan)
rstan_options(auto_write = TRUE)
options(mc.cores = parallel::detectCores())

overlap <- stan_model(file="overlapChol2.stan",model_name = "overlap")


library(MASS)


N = 300

mean_1 = c(0,0,0)
mean_2 = c(4,0,0)
var_f0 = 1
var_f2 = 1
var_dur = 1
F2.cor <- 0
F2.cov <- F2.cor*sqrt(var_f0)*sqrt(var_f2)
Sigma <- matrix(c(var_f0,F2.cov,0,F2.cov,var_f2,0,0,0,var_dur),3,3)
F2_1 = as.data.frame(round(mvrnorm(n= N, mean_1, Sigma),2))
F2_2 = as.data.frame(round(mvrnorm(n= N, mean_2, Sigma),2))
F2_1$cat = 1
F2_2$cat = 2
F2 = rbind(F2_1,F2_2)
names(F2) = c("F0","F2","Dur","cat")
F2$cat <- as.factor(F2$cat)
# qplot(data = F2,x=F0, y=F2,col=cat,geom="point")+theme_bw()+stat_ellipse()



stanData <- with(F2,
                 list(K = 2,
                      N = 600,
                      inds = rbind(c(1,300),c(301,40)),
                      D = 3,
                      nSims = 5000,
                      y = cbind(F0,F2,Dur)))

test <- sampling(overlap,stanData,chains=4,iter=4000)

summary(test,pars=c("omega_app","meanProb"))$summary
summary(test)$summary
