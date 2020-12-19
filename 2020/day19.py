from typing import List, Set, Dict
import re


def parse() -> (Dict[str, str], List[str]):
    file = open("day19.txt", "r")
    lines = file.read().splitlines()

    idx = 0
    while lines[idx] != '':
        idx += 1

    graph = {}
    for rule in lines[:idx]:
        from_node, to_node = rule.split(': ')
        graph[from_node] = to_node

    return graph, lines[idx + 1:]


def all_valid_messages(graph: Dict[str, str]) -> Set[str]:
    def dfs(current: str) -> List[str]:
        pattern = graph[current]

        # e.g. '"b"'
        match = re.findall(r'"(\w)"', pattern)
        if match:
            return match
        else:
            result = []
            # e.g. 2 3 | 3 2
            for sub_patten in pattern.split(' | '):
                current_results = ['']
                # e.g. e.g. 2 3
                for idx2 in sub_patten.split(' '):
                    partial_results = dfs(idx2)
                    current_results = [cr + pr for pr in partial_results for cr in current_results]
                result.extend(current_results)
            return result

    return set(dfs(current='0'))


def validate_message_part2(graph: Dict[str, str], message: str) -> bool:
    def dfs(current: str, m_idx: int) -> List[int]:
        """
        :param current: the current position in the graph
        :param m_idx: current index of the message
        :return: list of number of potential matches
        """

        if m_idx >= len(message):
            return []

        pattern = graph[current]

        # e.g. '"b"'
        match = re.findall(r'"(\w)"', pattern)
        if match:
            return [m_idx + 1] if match[0] == message[m_idx] else []
        else:
            all_matches = []
            # e.g. '2 3 | 3 2'
            for sub_patten in pattern.split(' | '):
                current_matches = [m_idx]
                # e.g. '2 3'
                for idx2 in sub_patten.split(' '):
                    new_matches = []
                    for current_match in current_matches:
                        new_matches.extend(dfs(idx2, current_match))
                    current_matches = new_matches
                all_matches.extend(current_matches)
            return all_matches

    return len(message) in dfs(current='0', m_idx=0)


indexed_rules, messages = parse()

part1_valid_messages = all_valid_messages(graph=indexed_rules)
part1 = sum([message in part1_valid_messages for message in messages])
print(part1)

indexed_rules['8'] = "42 | 42 8"
indexed_rules['11'] = "42 31 | 42 11 31"
part2 = sum([validate_message_part2(graph=indexed_rules, message=message) for message in messages])
print(part2)
