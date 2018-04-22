data {
    int<lower=1> K ; // number of vowel categories
    int<lower=1> N ; // number of data points
    int inds[K,2] ; // indices for different vowels
    int<lower=2> D ; // dimensionality of vowel data
    // int<lower=1,upper=K> vowel[N] ; // vowel label
    int<lower=1> nSims; // number of posterior simulations
    vector[D] y[N] ; // vowel measurements
}
parameters{
    vector[D] mu[K] ; // vowel means
    cholesky_factor_corr[D] L[K] ; // vowel correlation matrices
    vector<lower=0>[D] sigma[K] ; // sds
}
model{
    // implicit (flat) prior on mu
    // LKJ prior on correlations between dimensions because instability w/out it
    for (k in 1:K){
        //mu[k] ~ normal(0,50) ;
        sigma[k] ~ normal(0,50) ;
        L[k] ~ lkj_corr_cholesky(2.0) ;
    }
    for (i in 1:K){
        y[inds[i][1]:inds[i][2]] ~ multi_normal_cholesky(mu[i], diag_pre_multiply(sigma[i],L[i])) ;
    }
}
generated quantities{
    real omega_app ;
    real meanProb[2] ;
    // density_name_log(x,pars) is the format for evaluating (log) density at point.
    // seems to be vectorized?
    // probs <- multi_normal_cholesky_log(postSim,diag_pre_multiply(sigma,L)) ;
    {
        matrix[D,D] L_cov[K] ;
        // real meanProb[2] ;
        for (m in 1:K){
            L_cov[m] <- diag_pre_multiply(sigma[m],L[m]) ;
        }
        for (i in 1:K){
          real probs[nSims] ;
          vector[D] sims[nSims] ;
          for (j in 1:nSims){
            real prob_1 ;
            real prob_2 ;
            sims[j] <- multi_normal_cholesky_rng(mu[i], L_cov[i]) ;
            prob_1 <- exp(multi_normal_cholesky_log(sims[j],mu[1],L_cov[1])) ;
            prob_2 <- exp(multi_normal_cholesky_log(sims[j],mu[2],L_cov[2])) ;

            if (i == 1){
              probs[j] <- prob_2 / (prob_1+prob_2) ;
            }
            if (i == 2){
              probs[j] <- prob_1 / (prob_1+prob_2) ;
            }
          }
          meanProb[i] <- mean(probs) ;
        }
        omega_app <- sum(meanProb) ;
    }
}
