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

            abe = Man.new("abe")
            bob = Man.new("bob")
            col = Man.new("col")
            dan = Man.new("dan")
            ed = Man.new("ed")
            fred = Man.new("fred")
            gav = Man.new("gav")
            hal = Man.new("hal")
            ian = Man.new("ian")
            jon = Man.new("jon")

            men = {
                "abe" => abe,
                "bob" => bob,
                "col" => col,
                "dan" => dan,
                "ed" => ed,
                "fred" => fred,
                "gav" => gav,
                "hal" => hal,
                "ian" => ian,
                "jon" => jon,
            }


            abi = Woman.new("abi")
            bea = Woman.new("bea")
            cath = Woman.new("cath")
            dee = Woman.new("dee")
            eve = Woman.new("eve")
            fay = Woman.new("fay")
            gay = Woman.new("gay")
            hope = Woman.new("hope")
            ivy = Woman.new("ivy")
            jan = Woman.new("jan")

            women = {
                "abi" => abi,
                "bea" => bea,
                "cath" => cath,
                "dee" => dee,
                "eve" => eve,
                "fay" => fay,
                "gay" => gay,
                "hope" => hope,
                "ivy" => ivy,
                "jan" => jan,
            }

            abe.add_preferences([abi, eve, cath, ivy, jan, dee, fay, bea, hope, gay])
            bob.add_preferences([cath, hope, abi, dee, eve, fay, bea, jan, ivy, gay])
            col.add_preferences([hope, eve, abi, dee, bea, fay, ivy, gay, cath, jan])
            dan.add_preferences([ivy, fay, dee, gay, hope, eve, jan, bea, cath, abi])
            ed.add_preferences([jan, dee, bea, cath, fay, eve, abi, ivy, hope, gay])
            fred.add_preferences([bea, abi, dee, gay, eve, ivy, cath, jan, hope, fay])
            gav.add_preferences([gay, eve, ivy, bea, cath, abi, dee, hope, jan, fay])
            hal.add_preferences([abi, eve, hope, fay, ivy, cath, jan, bea, gay, dee])
            ian.add_preferences([hope, cath, dee, gay, bea, abi, fay, ivy, jan, eve])
            jon.add_preferences([abi, fay, jan, gay, eve, bea, dee, cath, ivy, hope])

            abi.add_preferences([bob, fred, jon, gav, ian, abe, dan, ed, col, hal])
            bea.add_preferences([bob, abe, col, fred, gav, dan, ian, ed, jon, hal])
            cath.add_preferences([fred, bob, ed, gav, hal, col, ian, abe, dan, jon])
            dee.add_preferences([fred, jon, col, abe, ian, hal, gav, dan, bob, ed])
            eve.add_preferences([jon, hal, fred, dan, abe, gav, col, ed, ian, bob])
            fay.add_preferences([bob, abe, ed, ian, jon, dan, fred, gav, col, hal])
            gay.add_preferences([jon, gav, hal, fred, bob, abe, col, ed, dan, ian])
            hope.add_preferences([gav, jon, bob, abe, ian, dan, hal, ed, col, fred])
            ivy.add_preferences([ian, col, hal, gav, fred, bob, abe, ed, jon, dan])
            jan.add_preferences([ed, hal, gav, abe, bob, jon, col, ian, fred, dan])

            Marriage.stable_match(men, women)

            expect(abe.engaged_to).to eq(ivy.name)
            expect(bob.engaged_to).to eq(cath.name)
            expect(col.engaged_to).to eq(dee.name)
            expect(dan.engaged_to).to eq(fay.name)
            expect(ed.engaged_to).to eq(jan.name)
            expect(fred.engaged_to).to eq(bea.name)
            expect(gav.engaged_to).to eq(gay.name)
            expect(hal.engaged_to).to eq(eve.name)
            expect(ian.engaged_to).to eq(hope.name)
            expect(jon.engaged_to).to eq(abi.name)

            expect(ivy.engaged_to).to eq(abe.name)
            expect(cath.engaged_to).to eq(bob.name)
            expect(dee.engaged_to).to eq(col.name)
            expect(fay.engaged_to).to eq(dan.name)
            expect(jan.engaged_to).to eq(ed.name)
            expect(bea.engaged_to).to eq(fred.name)
            expect(gay.engaged_to).to eq(gav.name)
            expect(eve.engaged_to).to eq(hal.name)
            expect(hope.engaged_to).to eq(ian.name)
            expect(abi.engaged_to).to eq(jon.name)
        end
    end
end
