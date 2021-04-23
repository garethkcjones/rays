#include "hittable_block.hh"

#include <cassert>
#include <memory>
#include <optional>
#include <utility>

#include "hittable.hh"
#include "hittable_aarect.hh"
#include "hittable_hitrecord.hh"
#include "material.hh"
#include "ray.hh"
#include "vec3.hh"

using namespace rays::hittable;
using rays::material::Material;

Block::Block(Vec3 const box_min,
             Vec3 const box_max,
             std::shared_ptr<Material> material)
{
	auto const [p0x, p0y, p0z] = box_min;
	auto const [p1x, p1y, p1z] = box_max;

	sides_.push_back(XyRect::new_hittable(p0x, p1x, p0y, p1y, p1z, material));
	sides_.push_back(XyRect::new_hittable(p0x, p1x, p0y, p1y, p0z, material));

	sides_.push_back(XzRect::new_hittable(p0x, p1x, p0z, p1z, p1y, material));
	sides_.push_back(XzRect::new_hittable(p0x, p1x, p0z, p1z, p0y, material));

	sides_.push_back(YzRect::new_hittable(p0y, p1y, p0z, p1z, p1x, material));
	sides_.push_back(YzRect::new_hittable(p0y, p1y, p0z, p1z, p0x,
		std::move(material)));

	assert(sides_.size() == 6);
}

auto Block::new_hittable(Vec3 const box_min,
                         Vec3 const box_max,
                         std::shared_ptr<Material> material)
	-> std::shared_ptr<Hittable>
{
	return std::make_shared<Block>(box_min, box_max, std::move(material));
}

auto Block::hit(Ray const &r, double const t_min, double const t_max) const
	noexcept -> std::optional<HitRecord>
{
	return sides_.hit(r, t_min, t_max);
}
