require_relative '../stable_marriage.rb'

RSpec.describe Marriage do
    describe "#stable_match" do
        it "creates a stable stable_match of women and men" do
            a = Man.new("a")
            b = Man.new("b")
            c = Man.new("c")
            d = Man.new("d")
            e = Man.new("e")

            men = {
                "a" => a,
                "b" => b,
                "c" => c,
                "d" => d,
                "e" => e,
            }

            l = Woman.new("l")
            m = Woman.new("m")
            n = Woman.new("n")
            o = Woman.new("o")
            p = Woman.new("p")

            women = {
                "l" => l,
                "m" => m,
                "n" => n,
                "o" => o,
                "p" => p,
            }

            a.add_preferences([o, m, n, l, p])
            b.add_preferences([p, n, m, l, o])
            c.add_preferences([m, p, l, o, n])
            d.add_preferences([p, m, o, n, l])
            e.add_preferences([o, l, m, n, p])

            l.add_preferences([d, b, e, c, a])
            m.add_preferences([b, a, d, c, e])
            n.add_preferences([a, c, e, d, b])
            o.add_preferences([d, a, c, b, e])
            p.add_preferences([b, e, a, c, d])

            # Marriage.stable_match returns the following men-women pair
            # [a, o], [b, p], [c, n], [d, m], [e, l]
            Marriage.stable_match(men, women)

            expect(a.engaged_to).to eq(o.name)
            expect(o.engaged_to).to eq(a.name)

            expect(b.engaged_to).to eq(p.name)
            expect(p.engaged_to).to eq(b.name)

            expect(c.engaged_to).to eq(n.name)
            expect(n.engaged_to).to eq(c.name)

            expect(d.engaged_to).to eq(m.name)
            expect(m.engaged_to).to eq(d.name)

            expect(e.engaged_to).to eq(l.name)
            expect(l.engaged_to).to eq(e.name)
        end
    end
end
