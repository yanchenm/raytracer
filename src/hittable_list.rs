use crate::hittable::{Hit, Hittable};

#[derive(Clone)]
pub struct HittableList<'a> {
    pub objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: &'a impl Hittable) {
        self.objects.push(object);
    }

    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut res: Option<Hit> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let hit_res = object.hit(r, t_min, closest_so_far);
            match hit_res {
                Some(hit) => {
                    closest_so_far = hit.t;
                    res = Some(hit);
                }
                None => continue,
            }
        }

        res
    }
}
