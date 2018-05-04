use na::{DVectorSliceMut, Real};

use joint::Joint;
use math::{Isometry, JacobianSliceMut, Translation, Vector, Velocity};
use solver::IntegrationParameters;

/// A joint that does not allow any relative degrees of freedom.
#[derive(Copy, Clone, Debug)]
pub struct FixedJoint<N: Real> {
    body_to_parent: Isometry<N>,
}

impl<N: Real> FixedJoint<N> {
    /// Create a joint that does not a allow any degrees of freedom between two body parts.
    /// 
    /// The descendent attached to this joint will have a position maintained to `pos_wrt_pody`
    /// relative to its parent.
    pub fn new(pos_wrt_body: Isometry<N>) -> Self {
        FixedJoint {
            body_to_parent: pos_wrt_body.inverse(),
        }
    }
}

impl<N: Real> Joint<N> for FixedJoint<N> {
    fn ndofs(&self) -> usize {
        0
    }

    fn body_to_parent(&self, parent_shift: &Vector<N>, body_shift: &Vector<N>) -> Isometry<N> {
        let parent_trans = Translation::from_vector(*parent_shift);
        let body_trans = Translation::from_vector(*body_shift);
        parent_trans * self.body_to_parent * body_trans
    }

    fn update_jacobians(&mut self, _: &Vector<N>, _: &[N]) {}

    fn jacobian(&self, _: &Isometry<N>, _: &mut JacobianSliceMut<N>) {}

    fn jacobian_dot(&self, _: &Isometry<N>, _: &mut JacobianSliceMut<N>) {}

    fn jacobian_dot_veldiff_mul_coordinates(
        &self,
        _: &Isometry<N>,
        _: &[N],
        _: &mut JacobianSliceMut<N>,
    ) {
    }

    fn integrate(&mut self, _: &IntegrationParameters<N>, _: &[N]) {}
    fn apply_displacement(&mut self, _: &[N]) {}

    fn jacobian_mul_coordinates(&self, _: &[N]) -> Velocity<N> {
        Velocity::zero()
    }

    fn jacobian_dot_mul_coordinates(&self, _: &[N]) -> Velocity<N> {
        Velocity::zero()
    }

    fn default_damping(&self, _: &mut DVectorSliceMut<N>) {}
}
