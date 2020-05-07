---
title: Solving Equations Using the Newton-Raphson Method
slug: solving-equations-using-the-newton-raphson-method
author: kenton
published: 2011-12-03T14:50:00-07:00
tags: [Python, Math]
summary: "Computers are great, but as it turns out—they're not always the smartest of folks. However, they are great at doing simple math! Today, I'll show you how to exploit these silicon monsters to do something that sometimes humans even fail at: solving a simple non-linear equation."
section: Math
---

Computers are great, but as it turns out—they're not always the smartest of folks. However, they are great at doing simple math! Today, I'll show you how to exploit these silicon monsters to do something that sometimes humans even fail at: solving a simple non-linear equation.

What do I mean by solving a non-linear equation? Well, try solving for $$x$$ in the following equation:

```katex
5x+\ln\left(x\right)-\sin\left(3\pi x\right)+x^{0.53}=0
```

It's not even likely that there **is** an analytical solution to this, but if there is the amount of work needed to solve it would simply be ludicrous. This is where numerical methods come in. Numerical methods allow complicated equations to be solved by repeatedly solving a series of smaller, easier-to-solve equations. They're limited in that they're **not exact** equations, but most often they do the job well enough regardless.

One the most famous (and simple) of numerical methods is the [Newton-Raphson](http://en.wikipedia.org/wiki/Newton's_method) method. This is a method for find the local roots of a real function. If you want more details about it, head over to Wikipedia, but basically the formula is like such:

```katex
x_{n+1}=x_n-\frac{f(x_n)}{f'(x_n)}
```

This simply means that you start with an initial guess, $$Xn$$, that you think is reasonably close to your answer (you can often get a good indication of this by plotting the function and seeing about where it hits 0). Next, you successively update your guess of what $$x$$ is by making adjustment based on the slope of the function. Finally, you stop improving your guess of what $$x$$ is once the error (or value of plugging in your estimated $$x$$ is in the function) is reasonably small.

To demonstrate, I've whipped up a simple program in Python which will solve the above problem. It's quite heavily commented, so hopefully you won't have any trouble following along. Here's the source code listing:

```python
#!/usr/bin/env python
from math import *

# the x function, effectively return the
# deviation from 0 for any give x
def function(x):
	return 5*x + log(x) - sin(3*pi*x) + x**0.53

# return the central-difference differential
# with a width of c centered about x
def dx(x, c):
	return (function(x+c) - function(x-c)) / (2*c)

# the Newton-Raphson method! For a given initial
# x value, width of the central-difference method,
# and maximum error desired in the results, will
# calculate the local zero of the function!
def Newton(x0, c, maxError):
	# initialize arrays
	x = [x0]
	e = [function(x0)]

	# now loop to solve!
	while fabs(e[len(e) - 1]) > maxError:
		# calculate the next x
		x.append(x[len(x) - 1] - (function(x[len(x) - 1]) / dx(x[len(x) - 1], c)))
		# update the error estimate
		e.append(function(x[len(x) - 1]))

	# and print out the results
	print 'i\tx\terror'
	print '---'
	for i in range(0, len(x)):
		print '%d\t%.3f\t%.3f' % (i+1, x[i], e[i])

# call the Newton-Raphson method with an initial
# guess of 0.25, a central difference width of 0.001
# and a maximum error of 0.001
startTime = time.time()
Newton(0.25, 0.001, 0.001)
endTime = time.time()
print 'Total process took %f seconds!' % (endTime - startTime)
```

Running this code results in the following:

    i	x	error
    ---
    1	0.250	-0.364
    2	0.272	0.010
    3	0.271	0.000
    Total process took 0.003000 seconds!

As you can see, the value of $$x$$ that solved the above equation to an accuracy of better than 0.001 in 3 ms! Note: it probably took even less time that that, but my system clock might not be that accurate / printing to the screen is a relatively slow business. In case you're curious, an $$x$$ value of 0.271 will make the above equation equal 0.

Pretty neat huh?
