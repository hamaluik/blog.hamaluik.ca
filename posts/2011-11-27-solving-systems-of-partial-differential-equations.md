---
title: Solving Systems of Partial Differential Equations
slug: solving-systems-of-partial-differential-equations
author: kenton
published: 2011-11-27T00:00:00-07:00
tags: [Math, Matlab]
section: Math
summary: "Systems of partial differential equations crop up all the time in engineering, especially when examining real-world complicated problems that vary in time (such as a ballistic trajectory with drag forces non-negligable), or in various process control systems (ex: relating flow conditions in systems of tanks with the height in those tanks)."
---

Systems of partial differential equations crop up all the time in engineering, especially when examining real-world complicated problems that vary in time (such as a ballistic trajectory with drag forces non-negligable), or in various process control systems (ex: relating flow conditions in systems of tanks with the height in those tanks).

An example system of partial differential equations may be given as:

```katex
\begin{aligned}
\frac{\partial x}{\partial t} &= 5xy + \sqrt{xt} + y^2 \\
\frac{\partial y}{\partial t} &= -2 \pi \cos(xt)
\end{aligned}
```

While these functions might look damn-near impossible to solve (and probably are analytically), these functions are a breeze for Matlab using the function `ode45` (at least, when numerically solving them). In order to use this function, you must first create a function in Matlab which emulates the above system of equations.

To do this, you'll have have to make some slight changes in notation to what you have above, since all the inputs (\(x\) _and_ \(y\) will be input in a single variable and all the outputs (\(dx/dt\), \(dy/dt\) will be output in a single variable. How is this done? Essentially all you do is group the variables into column matrices and then input / return those matrices. The input to the function becomes:

```katex
X = \left[x,y\right]
```

While the output from the function becomes:

```katex
\dot{X} = \left[\frac{\partial x}{\partial t},\frac{\partial y}{\partial t}\right]
```

We then simply create a function which takes in the matrix \(X\) and outputs the matrix \(\dot{X}\). Now when we're writing our function, whenever we need to write \(x\), we will just replace it with \(X\) (1) (which is the first element in the X matrix); whenever we want to write y, we will replace it with \(X\) (2) (which is the second element in the \(X\) matrix, which is y); whenever we want to write \(dx/dt\), we will just replace it with \(\dot{X}(1)\), etc. But what about the t input you ask? Well that's an additional parameter that gets input into our function, but we don't need to worry about packaging it into a function as it gets it's own parameter.

Without further ado, let's take a look at the function that we'll create to solve this set of differential equations:

```matlab
function [xd] = FunkySystem(t, x)
    % first create the column matrix of output
    % note - MUST be columnar!
    xd = zeros(2, 1);

    % now the first equation:
    xd(1) = 5 * x(1) * x(2) + sqrt(x(1)t) + x(2)^2;

    % and the second equation:
    xd(2) = -2 * pi * cos(x(1)t);
end
```

Save this code in the file `FunkySystem.m` in the current Matlab working directory.

See how this function takes in the parameters $$x$$, $$y$$ (in $$X$$), and $$t$$? It then takes those parameters and calculates the differentials we're looking for an returns them in $$\dot{X}$$. So, how do we now take this and calculate $$x$$ and $$y$$ over a given time frame? Using the aforementioned `ode45` of course! `ode45` takes the form:

```matlab
[t, x] = ode45(@function_name, [t0 tf], [initial_values])
```

So, for our functions above, we need the initial conditions, that is, what are the values of x and y at time t=0? For simplicity's sake, lets use the following initial conditions:

```katex
x(0) = 0 \\
y(0) = 1
```

Note, for this method to work, we **must** be using initial-condition differential equations. Also, we need to know the time interval we want to examine things over. Let's try going up to 5 seconds. Thus, to solve with these conditions, all we have to do is:

```matlab
[t, x] = ode45(@FunkySystem, [0 5], [0 1]);
```

Notice how we input the initial conditions (at t0) combined together as a 1x2 matrix. Now, we can examine the functions x(t) and y(t) which will be the first and second columns of the output X, respectively.

```matlab
% plot the x curve
subplot(2, 1, 1);
plot(t, x(:,1));
xlabel('Time, t (s)', 'interpreter', 'latex');
ylabel('x(t)', 'interpreter', 'latex')

% plot the y curve
subplot(2, 1, 2);
plot(t, x(:,2));
xlabel('Time, t (s)', 'interpreter', 'latex');
ylabel('y(t)', 'interpreter', 'latex')
```

Matlab solved this almost isntantly, and as you can see, they're rather weird functions. This is simply becuase I made up two partial differential functions completely randomly.

## Systems With Second (Or Higher) Order Differential Equations

For systems with second-order partial differential equations as in the function:

```katex
m \ddot{x} + c \dot{x} + k x = f(t)
```

This method does not work straight away (we need first-order differential equations to be solved with `ode45`. Luckily, it is easy and possible to write a second-order differential in terms of a system of first order differentials by using a simple substitution. That is, create a new variable, $$u$$, and define it as such:

```katex
u = \dot{x}
```

Thus our main equation becomes:

```katex
m \dot{u} + c u + k x = f(t)
```

```katex
u = \dot{x}
```

Which is now a system of first-oder partial differential equations. Now rewrite these in their appropriate forms to be coded into the function:

```katex
\dot{u} = \frac{1}{m}\left(f(t) - cu - kx\right)
```

```katex
\dot{x} = u
```

Now, in order to solve the system, we will need two initial conditions (one for each equation):

```katex
x(0) = x_0
```

```katex
\dot{x}(0) = u(0) = \dot{x}_0
```

Now use these equations and initial conditions to solve away as shown before!
