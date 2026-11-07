impl crate::io::Io for TutPairGateFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }
impl crate::io::Io for TutPairParkFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }
impl crate::io::Io for TutPairYardFrame { fn w(&self, o: &mut crate::io::W) { self.at.w(o); } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default(); s.at = crate::io::Io::r(i); s } }
impl crate::io::Io for TutPairVars { fn w(&self, o: &mut crate::io::W) {  } fn r(i: &mut crate::io::R) -> Self { let mut s = Self::default();  s } }
