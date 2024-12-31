#include <pybind11/pybind11.h>
#include <pybind11/stl.h>
#include "rights.hpp"

namespace py = pybind11;

PYBIND11_MODULE(rights_cpp, m) {
    py::class_<Move>(m, "Move")
        .def(py::init<std::pair<int,int>, std::pair<int,int>, bool>())
        .def_readwrite("start", &Move::start)
        .def_readwrite("end", &Move::end)
        .def_readwrite("promotion", &Move::promotion);
        
    m.def("pawn", &pawn_moves, "Generate pawn moves");
    m.def("knight", &knight_moves, "Generate knight moves");
    m.def("bishop", &bishop_moves, "Generate bishop moves",
          py::arg("board"), py::arg("turn"), py::arg("equal") = -1);
    m.def("rook", &rook_moves, "Generate rook moves",
          py::arg("board"), py::arg("turn"), py::arg("equal") = -1);
    m.def("queen", &queen_moves, "Generate queen moves");
    m.def("king", &king_moves, "Generate king moves");
} 