package aoc;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class Day22 {

	public static void main(String[] args) throws IOException {
		File input = new File("input").getAbsoluteFile();
		if (!input.exists() || input.isDirectory()) {
			System.out.println(input.getPath() + " does not exist!");
			input = new File(input.getPath() + ".txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.out.println(input.getPath() + " does not exist!");
			input = new File(input.getParentFile().getParent(), "input");
			input = new File(input, "Day22.txt");
			System.out.println("trying " + input.getPath() + " instead.");
		}

		if (!input.exists() || input.isDirectory()) {
			System.err.println("None of the expected inputs exist!");
			return;
		} else {
			System.out.println("Using input file " + input.getPath() + ".");
		}

		List<Integer> origPlayer1cards = new ArrayList<>();
		List<Integer> origPlayer2cards = new ArrayList<>();
		boolean player2 = false;
		for (String line : Files.readAllLines(input.toPath())) {
			if (line.isEmpty())
				continue;

			if (line.startsWith("Player")) {
				if (line.endsWith("2:"))
					player2 = true;

				continue;
			}

			if (player2) {
				origPlayer2cards.add(Integer.parseInt(line));
			} else {
				origPlayer1cards.add(Integer.parseInt(line));
			}
		}

		List<Integer> player1cards = new ArrayList<>(origPlayer1cards);
		List<Integer> player2cards = new ArrayList<>(origPlayer2cards);
		while (player1cards.size() > 0 && player2cards.size() > 0) {
			int player1card = player1cards.get(0);
			player1cards.remove(0);
			int player2card = player2cards.get(0);
			player2cards.remove(0);

			if (player1card > player2card) {
				player1cards.add(player1card);
				player1cards.add(player2card);
			} else {
				player2cards.add(player2card);
				player2cards.add(player1card);
			}
		}

		int winner = 0;
		int winningScore = 0;
		if (player1cards.size() > 0) {
			winner = 1;
			for (int i = player1cards.size(); i > 0; i--) {
				winningScore += player1cards.get(player1cards.size() - i) * i;
			}
		} else {
			winner = 2;
			for (int i = player2cards.size(); i > 0; i--) {
				winningScore += player2cards.get(player2cards.size() - i) * i;
			}
		}

		System.out.printf("Player %d won part 1 with a score of %d.%n", winner, winningScore);

		player1cards = new ArrayList<>(origPlayer1cards);
		player2cards = new ArrayList<>(origPlayer2cards);
		winner = recursiveGame(player1cards, player2cards);
		winningScore = 0;
		if (winner == 1) {
			for (int i = player1cards.size(); i > 0; i--) {
				winningScore += player1cards.get(player1cards.size() - i) * i;
			}
		} else {
			for (int i = player2cards.size(); i > 0; i--) {
				winningScore += player2cards.get(player2cards.size() - i) * i;
			}
		}

		System.out.printf("Player %d won part 2 with a score of %d.%n", winner, winningScore);
	}

	private static int recursiveGame(List<Integer> player1cards, List<Integer> player2cards) {
		Set<List<List<Integer>>> states = new HashSet<>();
		while (player1cards.size() > 0 && player2cards.size() > 0) {
			List<List<Integer>> state = new ArrayList<>();
			state.add(new ArrayList<>(player1cards));
			state.add(new ArrayList<>(player2cards));
			if (states.contains(state))
				return 1;

			states.add(state);

			int player1card = player1cards.get(0);
			player1cards.remove(0);
			int player2card = player2cards.get(0);
			player2cards.remove(0);

			int winner = 0;
			if (player1cards.size() >= player1card && player2cards.size() >= player2card) {
				winner = recursiveGame(new ArrayList<>(player1cards.subList(0, player1card)),
						new ArrayList<>(player2cards.subList(0, player2card)));
			} else {
				winner = player1card > player2card ? 1 : 2;
			}

			if (winner == 1) {
				player1cards.add(player1card);
				player1cards.add(player2card);
			} else {
				player2cards.add(player2card);
				player2cards.add(player1card);
			}
		}

		if (player1cards.size() > 0) {
			return 1;
		} else {
			return 2;
		}
	}
}
