#include "hittable_hitrecord.hh"

#include <memory>
#include <utility>

#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

rays::hittable::HitRecord::HitRecord(
	Ray const &r,
	Vec3 const p,
	Vec3 const outward_normal,
	double const t,
	std::shared_ptr<material::Material> material
) noexcept:
	p_{p},
	normal_{outward_normal},
	t_{t},
	material_{std::move(material)},
	front_face_{dot(r.direction(), outward_normal) < 0.0}
{
	if (!front_face_)
		normal_ = -outward_normal;
}
