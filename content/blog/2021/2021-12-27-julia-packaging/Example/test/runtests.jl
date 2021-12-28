using Test
@test [1, 2] + [2, 1] == [3, 3]
@test_throws BoundsError [1, 2, 3][4]
"""
pkg> activate .
(Example) pkg> test
"""

