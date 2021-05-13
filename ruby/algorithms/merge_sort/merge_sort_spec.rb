require_relative 'merge_sort'

RSpec.describe do
    describe "#merge_sort" do
        it "sorts an array whose length is even" do
            a = [4, 2, 3, 1]
            merge_sort(a)

            expect(a).to eq([1, 2, 3, 4])
        end

        it "sorts an array whose length is odd" do
            a = [4, 6, 5, 7, 3]
            merge_sort(a)

            expect(a).to eq([3, 4, 5, 6, 7])
        end

        it "sorts a very short array" do
            a = [3, 2, 1]
            merge_sort(a)

            expect(a).to eq([1, 2, 3])
        end

        it "does not break on an array containing one element" do
            a = [1]
            merge_sort(a)

            expect(a).to eq([1])
        end

        it "does not break on an empty array" do
            a = []
            merge_sort(a)

            expect(a).to eq([])
        end
    end
end