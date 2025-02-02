<!-- # Model Predictive Control for an Autonomous Car -->

During my time at [Fuel Fighter NTNU](https://www.fuelfighter.no/) I among other things developed a Nonlinear Model Predictive Controller (NMPC) to control an autonomous car. Fuel Fighter NTNU is an voluntary student organisation at (Norwegian University of Science and Technology) which develops one of the worlds most energy effiecient and autonomous cars for the student competition [Shell Eco-Marathon](https://www.shellecomarathon.com/). Together with Erik Thallaug Fagerli, I develop an Nonliner MPC for path following and parking of the autonomous Fuel Fighter car. Erik wrote his master thesis about the controller which can be found at [NTNU Open](https://hdl.handle.net/11250/3019945). The MPC was successfully used in the 2022 Shell Eco-Marathon competition, where Fuel Fighter NTNU placed 3rd. The MPC uses ROS to send and get information. The code and more information about the MPC can be found [HERE](https://github.com/TorBorve/mpc_local_planner).

<br/>
<div align="center">
<img src="../static/mpc_demo.gif" width="40%">
<br>
<figcaption align="center">Fig. 1: Demo of MPC using gazebo simulator and audibot car</figcaption>
</div>
<br/>

## About The MPC

A crucial part of an MPC is the model. The car is modelled using the [bicycle model](https://dingyan89.medium.com/simple-understanding-of-kinematic-bicycle-model-81cac6420357). This model assumes that the car only has two wheels which are located at center front and rear, respectively. One can thus determine the turning radius and turning rate based on the distance between the front and rear (known as baseline), the speed of the vehicle, and the steering angle. See Fig. 2 bellow. For speed a linear model for acceleration from throttle was used. Using this model we can describe how the car will move given different throttle and steering commands. The represent the path mathematically the discrete path was fitted using a third order polynomial, see the blue line in Fig. 1. 

<br/>
<div align="center">
<img src="../static/bicycle_model.webp" width="50%">
<br>
<figcaption align="center">Fig. 2: Bicycle model geometry from Yan Ding</figcaption>
</div>
<br/>


Given a model of the car and path it remains to state what we want the car to do mathematically, i.e. a cost function. The main parts of the cost function is the cross-track error, which is the distance from the car to the path/road, and the heading error, which is the angle between the direction the car is pointing and the direction the path is heading. We want both of these to be small. The MPC then attempts to find the optimal steering angle and throttle value such that we minimize the cost function over time. This is done by calculation what the future cost will be given different steering angles and throttle values over time. Then it does the fist part of the plan, after which it recalculates the plan and only does the first part of it. This is known as receeding horizon as seen in Fig. 3. Solving these optimization problems can be very challenging, especially since the problem in this case is nonlinear. Therefore, [ACADOS](https://github.com/acados/acados) was used. ACADOS is a library which makes is "simple" to solve these kinds of problems and does it very fast.

<br/>
<div align="center">
<img src="../static/mpc_diagram.png" width="60%">
<br>
<figcaption align="center">Fig. 3: MPC receeding horizon</figcaption>
</div>
<br/>

## Final Thoughts

The MPC works well for several senarios, however looking back I would do several things different. This includes, but is not limited to the following points:

- **Nonlinear**: The fact the problem becomes makes the optimisation problem much harder and you loose almost all guarantees of optimal solutions. If I were to do the project again I would probably transform the problem such that a linear approximation could be used. This would help ensure that the solutions obtained are acctually good.
- **Polynomial**: Using a polynomial to approximate the path is very simple and is not very good in terms of the fit it provides. Thus, could use something like a spline or Lagrange polynomial, which would likely be better. I tried to use a spline, however at the time ACADOS had limited support for updating the spline (at least not well documented...).
- **KISS**: Keep it simple stupid. As mentioned using a nonlinear solver is complicated and you have no guarantees for it to work. In heinsight it would be better to just use a simpel path following algorithm such as Pure Pursuit or Stanley controller, however it is not as cool nor impressive ;)

If you have questions about the project feel free to reach out to me :)