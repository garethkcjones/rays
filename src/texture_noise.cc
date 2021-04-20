#include "texture_noise.hh"

#include <memory>
#include <random>

#include "colour.hh"
#include "texture.hh"
#include "vec3.hh"

using namespace rays::texture;

Noise::Noise(std::default_random_engine &rand_eng, double const scale):
	noise_{rand_eng},
	scale_{scale}
{
}

auto Noise::new_texture(std::default_random_engine &rand_eng,
                        double const scale)
	-> std::shared_ptr<Texture>
{
	return std::make_shared<Noise>(rand_eng, scale);
}

auto Noise::value(double /*u*/, double /*v*/, Vec3 const p) const -> Colour {
	return Colour{1.0, 1.0, 1.0} * noise_.turb(scale_ * p);
}
