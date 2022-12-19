% NEWUOAM provides the MATLAB implementation of M. J. D. Powell's NEWUOA algorithm described in
%
% M. J. D. Powell, The NEWUOA software for unconstrained optimization
% without derivatives, In Large-Scale Nonlinear Optimization, eds. G. Di
% Pillo and M. Roma, pages 255--297, Springer, New York, US, 2006
%
% NEWUOA seeks the least value of a function of many variables, by a
% trust region method that forms quadratic models by interpolation.
% There can be some freedom in the interpolation conditions, which is
% taken up by minimizing the Frobenius norm of the change to the second
% derivative of the quadratic model, beginning with a zero matrix.
%
% Coded by Zaikun Zhang in July 2020 based on Powell's Fortran 77 code
% and the NEWUOA paper.
%
% Last Modified: Thursday, July 22, 2021 AM10:56:17

function [x, f, exitflag, nf, xhist, fhist] = newuoam(calfun, x, rhobeg, rhoend, eta1, eta2, gamma1, gamma2, ftarget, maxfun, npt, iprint, maxhist, output_xhist, debugflag)
	% A detailed introduction to the arguments is as follows.
	% N.B.: RP and IK are defined in the module CONSTS_MOD. See consts.F90 under
	% the directory name "common". By default, RP = kind(0.0D0) and IK = kind(0).
	% Therefore, REAL(RP) is the double-precision real, and INTEGER(IK) is the
	% default integer. For ADVANCED USERS, RP and IK can be defined by specifying
	% __REAL_PRECISION__ and __INTEGER_KIND__ in common/ppf.h. Use the default if
	% you are unsure.
	%
	% CALFUN
	%   Input, subroutine.
	%   CALFUN(X, F) should evaluate the objective function at the given
	%   REAL(RP) vector X and set the value to the REAL(RP) scalar F. It
	%
	% X
	%   Input and outout, REAL(RP) vector.
	%   As an input, X should be an N dimensional vector that contains the
	%   initial values of the variables, N being the dimension of the problem.
	%   As an output, X will be set to an approximate minimizer.
	%
	% F
	%   Output, REAL(RP) scalar.
	%   F will be set to the objective function value of the X at exit.
	%
	% NF
	%   Output, INTEGER(IK) scalar.
	%   NF will be set to the number of function evaluations at exit.
	%
	% RHOBEG, RHOEND
	%   Inputs, REAL(RP) scalars, default: RHOBEG = 1, RHOEND = 10^-6.
	%   RHOBEG and RHOEND must be set to the initial and final values of a
	%   trust region radius, so both must be positive with RHOEND <= RHOBEG.
	%   Typically RHOBEG should be about one tenth of the greatest expected
	%   change to a variable, and RHOEND should indicate the accuracy that is
	%   required in the final values of the variables.
	%
	% FTARGET
	%   Input, REAL(RP) scalar, default: - Infinity.
	%   FTARGET is the target function value. The algorithm will terminate
	%   when a point withi a function value <= FTARGET is found.
	%
	% MAXFUN
	%   Input, INTEGER(IK) scalar, default: consts(maxfun_dim_dft)*N with
	%   consts(maxfun_dim_dft) defined in the module CONSTS_MOD (see consts.F90 in
	%   the directory named "common").
	%   MAXFUN is the maximal number of function evaluations.
	%
	% NPT
	%   Input, INTEGER(IK) scalar, default: 2N + 1.
	%   NPT is the number of interpolation conditions for each trust region
	%   model. Its value must be in the interval [N+2, (N+1)(N+2)/2].
	%
	% IPRINT
	%   Input, INTEGER(IK) scalar, default: 0.
	%   The value of IPRINT should be set to 0, 1, -1, 2, -2, 3, or -3, which
	%   controls how much information will be printed during the computation:
	%   0: there will be no printing;
	%   1: a message will be printed to the screen at the return, showing the
	%      best vector of veriables found and its objective function value;
	%   2: in addition to 1, each new value of RHO is printed to the screen,
	%      with the best vector of variables so far and its objective function
	%      value;
	%   3: in addition to 2, each function evaluation with its variables will
	%      be printed to the screen;
	%   -1, -2, -3: the same information as 1, 2, 3 will be printed, not to
	%     the screen but to a file named NEWUOA_output.txt; the file will be
	%     created if it does not exist; the new output will be appended to
	%     the end of this file if it already exists. Note that IPRINT = -3 can
	%     be costly in terms of time and space.
	%
	% ETA1, ETA2, GAMMA1, GAMMA2
	%   Input, REAL(RP) scalars, default: ETA1 = 0.1, ETA2 = 0.7, GAMMA1 = 0.5,
	%   and GAMMA2 = 2.
	%   ETA1, ETA2, GAMMA1, and GAMMA2 are parameters in the updating scheme
	%   of the trust region radius as detailed in the subroutine TRRAD in
	%   trustregion.f90. Roughly speaking, the trust region radius is contracted
	%   by a factor of GAMMA1 when the reduction ratio is below ETA1, and
	%   enlarged by a factor of GAMMA2 when the reduction ratio is above ETA2.
	%   It is required that 0 < ETA1 <= ETA2 < 1 and 0 < GAMMA1 < 1 < GAMMA2.
	%   Normally, ETA1 <= 0.25. It is NOT recommended to set ETA1 >= 0.5.
	%
	% XHIST, FHIST, MAXHIST
	%   XHIST: Output, ALLOCATABLE rank 2 REAL(RP) array;
	%   FHIST: Output, ALLOCATABLE rank 1 REAL(RP) array;
	%   MAXHIST: Input, INTEGER(IK) scalar, default: MAXFUN
	%   XHIST, if present, will output the history of iterates, while FHIST,
	%   if present, will output the history function values. MAXHIST should
	%   be a nonnegative integer, and XHIST/FHIST will output only the last
	%   MAXHIST iterates and/or the corresponding function values. Therefore,
	%   MAXHIST = 0 means XHIST/FHIST will output nothing, while setting
	%   MAXHIST = MAXFUN ensures that  XHIST/FHIST will output all the history.
	%   If XHIST is present, its size at exit will be (N, min(NF, MAXHIST));
	%   if FHIST is present, its size at exit will be min(NF, MAXHIST).
	%
	%   Important Notice:
	%   Setting MAXHIST to a large value can be costly in terms of memory.
	%   For instance, if N = 1000 and MAXHIST = 100, 000, XHIST will take
	%   reset to a smaller value if the memory needed for XHIST and/or FHIST
	%   exceeds consts(maxmemory) defined in CONSTS_MOD (see consts.F90 under the
	%   directory named "common"; default: 2GB). Use XHIST, FHIST, and MAXHIST
	%   with caution%%%
	%
	% INFO
	%   Output, INTEGER(IK) scalar.
	%   INFO is the exit flag. It can be set to the following values defined
	%   in the module INFO_MOD (see info.F90 under the directory named "common"):
	%   infos(small_tr_radius): the lower bound for the trust region radius is reached;
	%   infos(ftarget_achieved): the target function value is reached;
	%   infos(trsubp_failed): a trust region step failed to reduce the quadratic model;
	%   infos(maxfun_reached): the objective function has been evaluated MAXFUN times;
	%   infos(nan_x): NaN occurs in x;
	%   infos(nan_inf_f): the objective function returns NaN or nearly infinite value;
	%   infos(nan_model): NaN occurs in the models.

	% Solver-specific modules

	% Dummy variables

	% Local variables

	% Get size.

    % Replace any NaN or Inf in X by 0.
    x(isnan(x) | isinf(x)) = 0;

	%-------------------- Call NEWUOB, which performs the real calculations. --------------------------%
    maxfhist = maxhist;
    if output_xhist == 0
        maxxhist = 0;
    else
        maxxhist = maxhist;
    end
	[x, nf, f, fhist, xhist, exitflag] = newuob(calfun, iprint, maxfun, npt, eta1, eta2, ftarget, gamma1, gamma2, rhobeg, rhoend, x, maxfhist, maxxhist, debugflag);
	%--------------------------------------------------------------------------------------------------%

	% Write the outputs.

	% Copy XHIST to XHIST if needed.
    % The SAFEALLOC line is removable in F2003.
    xhist = xhist(:, 1:min(nf, maxxhist));
    % N.B.:
    % 0. Allocate XHIST as long as it is present, even if MAXXHIST = 0;
    % otherwise, it will be illeagle to enquire XHIST after exit.
    % 1. Even though Fortran 2003 supports automatic (re)allocation of
    % allocatable arrays upon intrinsic assignment, we keep the line of
    % are still not standard-compliant in this respect.
    % 3. When MAXXHIST > NF, which is the normal case in practice,
    % XHIST contains GARBAGE in XHIST(:, NF + 1 : MAXXHIST).
    % Therefore, we MUST cap XHIST at min(NF, MAXXHIST) so that XHIST
    % cointains only valid history. For this reason, there is no way to
    % avoid allocating two copies of memory for XHIST unless we declare
    % it to be a POINTER instead of ALLOCATABLE.

    % The SAFEALLOC line is removable in F2003.
    fhist = fhist(1:min(nf, maxfhist));
    % The same as XHIST, we must cap FHIST at min(NF, MAXFHIST).

end
