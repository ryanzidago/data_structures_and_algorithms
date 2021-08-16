class Person
    attr_accessor :name, :preferences, :engaged_to
    def initialize(name = nil, preferences = [])
        @name = name
        @preferences = preferences
        @engaged_to = nil
    end

    def add_preferences(preferences)
        @preferences = preferences.map { |person| person.name }
    end
end

class Man < Person
end

class Woman < Person
end

# Gale-Shapley algorithm in pseudocode:
#
# algorithm stable_matching is
#     Initialize all m ∈ M and w ∈ W to free
#     while ∃ free man m who still has a woman w to propose to do
#         w := first woman on m's list to whom m has not yet proposed
#         if w is free then
#             (m, w) become engaged
#         else some pair (m', w) already exists
#             if w prefers m to m' then
#                 m' becomes free
#                 (m, w) become engaged
#             else
#                 (m', w) remain engaged
#             end if
#         end if
#     repeat

class Marriage
    def self.stable_match(men, women)
        available_men = men_who_are_not_engaged_and_havent_proposed_to_every_woman(men)
        while !available_men.empty?
            man = available_men.first()
            woman_name = man.preferences.shift()
            woman = women[woman_name]

            unless woman.engaged_to
                man.engaged_to = woman.name
                woman.engaged_to = man.name
            else
                _man_name = woman.engaged_to
                if woman.preferences.index(_man_name) > woman.preferences.index(man.name)
                    _man = men[_man_name]
                    _man.engaged_to = nil
                    _man.preferences.delete(woman.name)

                    man.engaged_to = woman.name
                    woman.engaged_to = man.name
                end
            end
            available_men = men_who_are_not_engaged_and_havent_proposed_to_every_woman(men)
        end
    end

    def self.men_who_are_not_engaged_and_havent_proposed_to_every_woman(men)
        men.values.filter { |man| !man.engaged_to and !man.preferences.empty? }
    end
end

# What happens if you add bisexual individuals / or more men than women and vice versa?
# What happens if some individuals do not which to be married?
# What happens if the women propose first?
# What happens if the men/women propose first alternatively?
# What happens if we change the order of preferences? Does it result in the same stable matching?
